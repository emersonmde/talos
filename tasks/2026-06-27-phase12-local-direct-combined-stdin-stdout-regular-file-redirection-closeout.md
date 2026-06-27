# Phase 12 Local Direct Combined Stdin Stdout Regular-File Redirection Closeout

Task: phase12-local-direct-combined-stdin-stdout-regular-file-redirection-closeout-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Reviewed the accepted direct combined stdin/stdout regular-file redirection
  core task, classification, evidence map, QEMU/substitute transcript,
  regression summaries, and project docs.
- Reconciled the exact accepted witness
  '/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt' against retained
  descriptor-backed VFS stdin source, volatile VFS stdout sink, userspace
  report, descriptor-backed readback, restoration, and fail-closed evidence.
- Kept the accepted surface limited to the direct path-form combined witness.
  The next mechanically objective same-lane task is the already queued
  bare-name combined stdin/stdout core because direct combined evidence is
  accepted and committed.
- Did not start bare-name combined implementation, arbitrary input/output
  paths, append, stderr combined forms, pipeline-output redirection,
  persistent writable filesystem behavior, live networking/SSH, Pi 5 hardware
  action, generated-root retry, or phase transition work.

## Findings

- not-an-issue: the core task's accepted witness is exactly
  '/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt'; retained evidence
  records child-only fd0 source-route 'initramfs:/etc/banner.txt', child-only
  fd1 target-route 'volatile-vfs:/tmp/stdin-report.txt', fd2 inherited as
  stdio output, and shell-restored=true for both descriptor mutations.
- not-an-issue: retained evidence records userspace '/bin/stdin' reading
  'Talos initramfs fixture' from fd0, writing its report through redirected fd1,
  descriptor-backed 'cat /tmp/stdin-report.txt' readback, closed loader
  temporary descriptors, and coherent waitpid/laststatus observations.
- not-an-issue: retained fail-closed evidence keeps unsupported direct combined
  forms outside the accepted surface: output-first ordering, spaced input
  grammar, /dev/null input, explicit 1> output, append output, stderr output,
  and arbitrary output paths.
- fixed: roadmap, Phase 12, and early POSIX docs now record the accepted direct
  combined closeout frontier and selected bare-name combined core.
- deferred: fixed-/bin bare-name combined redirection remains separate
  implementation work. Arbitrary input/output paths, append in combined forms,
  stderr combined forms, pipeline-output redirection/append, persistent
  writable filesystem behavior, PATH/current-directory search, command lookup
  beyond bounded /bin, arbitrary shell grammar, live networking/SSH,
  Pi 5 hardware proof, generated-root retry, and phase transition remain
  outside this closeout.

## Evidence

- static inspection:
  tasks/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core.md.
- task-owned JSON inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/evidence-map.json.
- QEMU/substitute transcript inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/qemu-local-shell-direct-combined-stdin-stdout-redirection-smoke.log.
- project doc inspection: docs/src/roadmap.md,
  docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-closeout/evidence-map.json.

## Validation

- 'jq empty
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/classification.json
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-closeout/evidence-map.json'
  passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed.

## Result

The accepted direct combined stdin/stdout regular-file redirection frontier is
reconciled against retained evidence and docs. The next mechanically objective
task is
phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core-20260627.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
