# Phase 10 Pi 5 Rootinfo Tail-Stable Source Response Core

Task id: phase10-pi5-rootinfo-tail-stable-source-response-core-20260618

Status: accepted

Classification:
command0-tail-stable-source-response-core-local-source

Evidence level: static source inspection, no_std unit tests under QEMU,
proof-helper fixture evidence, task-owned JSON evidence, docs build, and diff
checks.

## Goal

Make the rootinfo response retain source=firmware-initramfs and
reason=valid-artifact in a bounded tail-stable position so the next Pi 5
command0 proof can evaluate source-response retention after accepted command0
input delivery.

## Result

The rootinfo command still reports the generated-root selection from
initramfs::generated_root_selection_report(), but now writes the source and
reason fields at the end of the single generated-root response line after the
path fields. This preserves responses=1 for the command-loop proof while
making a tail capture that starts near path=/generated/manifest.txt retain the
source and reason fragments before the command0 dispatch/ready markers.

The Pi 5 command0 source-response discriminator now accepts same-command0
evidence when the retained text contains source/reason plus command0
line/rootinfo, dispatch command=0 status=handled, responses=1, and ready
command=1. It rejects dispatch-only, source-only, later-command, stale,
truncated, or non-UTF8 evidence.

selected_next_task is
phase10-pi5-rootinfo-tail-stable-source-response-pi5-proof-20260618.

## Findings

- fixed: moved source and reason to the tail of the rootinfo generated-root
  response while preserving the generated-root selection report source.
- fixed: added a unit test for the tail-stable rootinfo response format.
- fixed: added a task-owned command0 source-response discriminator that accepts
  same-command0 tail-stable source/reason evidence and rejects partial/stale
  shapes.
- fixed: updated the existing generated-root command-input proof review helper
  so source/reason do not have to appear immediately after the generated-root
  prefix.
- deferred: Pi 5 hardware source-response acceptance remains for the selected
  serialized proof task.
- rejected: generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Evidence

- Classification:
  tasks/evidence/2026-06-18-phase10-pi5-rootinfo-tail-stable-source-response-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-rootinfo-tail-stable-source-response-core/evidence-map.json.
- Source changes: src/local_command_loop.rs,
  scripts/rpi5-command0-tail-stable-source-response-discriminator.sh,
  scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh,
  and scripts/rpi5-generated-root-command-input-proof-review.sh.

## Acceptance Check

- Existing rootinfo command still reports the generated-root selection from
  initramfs::generated_root_selection_report(): satisfied by static source
  inspection and unit test.
- Command response includes source/reason in a bounded tail-stable position:
  satisfied by unit test and source inspection.
- Local/QEMU or unit evidence covers generated-root output: satisfied by
  cargo -Zjson-target-spec test --quiet under QEMU.
- Pi 5 proof helper expectations reject stale/partial evidence and accept only
  same-command0 response retention: satisfied by discriminator fixture evidence.
- Task record documents findings with disposition: satisfied.
- selected_next_task is the serialized Pi 5 proof task: satisfied.
- Rejected claims include source-response hardware acceptance, generated-root
  command-input success, storage, networking, SSH, Phase 11/12 expansion, and
  phase transition: satisfied.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass, 532 no_std tests under QEMU.
  Initial run without the QEMU tool path failed with qemu-system-aarch64 not
  found; rerun with the documented PATH passed.
- Proof-helper positive/negative fixture evidence: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-rootinfo-tail-stable-source-response-pi5-proof-20260618 on the
next worker wake if dependencies remain satisfied and the hardware lock is
unlocked/restored. Do not claim source-response hardware acceptance or
generated-root command-input success from this local/source task.
