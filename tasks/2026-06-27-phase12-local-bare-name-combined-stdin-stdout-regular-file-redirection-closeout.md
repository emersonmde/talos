# Phase 12 Local Bare-Name Combined Stdin Stdout Regular-File Redirection Closeout

Task: phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-closeout-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Reviewed the accepted bare-name combined stdin/stdout regular-file
  redirection core task, classification, evidence map, QEMU/substitute
  transcript, regression summaries, and project docs.
- Reconciled the exact accepted witness
  'stdin </etc/banner.txt >/tmp/stdin-report.txt' against retained direct
  combined evidence, bounded fixed-/bin command resolution, descriptor-backed
  VFS stdin source, volatile VFS stdout sink, userspace report, readback,
  restoration, and fail-closed boundaries.
- Kept the accepted surface limited to the fixed-/bin bare-name combined
  witness. The next mechanically objective same-lane task is the already
  queued combined stdin/stdout frontier checkpoint because direct and
  bare-name combined evidence are accepted and committed.
- Did not implement runtime feature changes, arbitrary input/output paths,
  append, stderr combined forms, pipeline-output redirection, persistent
  writable filesystem behavior, live networking/SSH, Pi 5 hardware action,
  generated-root retry, or phase transition work.

## Findings

- not-an-issue: the core task's accepted witness is exactly
  'stdin </etc/banner.txt >/tmp/stdin-report.txt'; retained evidence records
  bounded fixed-/bin resolution to '/bin/stdin', child-only fd0 source-route
  'initramfs:/etc/banner.txt', child-only fd1 target-route
  'volatile-vfs:/tmp/stdin-report.txt', fd2 inherited as stdio output, and
  shell-restored=true for the descriptor mutations.
- not-an-issue: retained evidence records userspace '/bin/stdin' reading
  'Talos initramfs fixture' from fd0, writing its report through redirected
  fd1, descriptor-backed 'cat /tmp/stdin-report.txt' readback, closed loader
  temporary descriptors, and coherent waitpid/laststatus observations.
- not-an-issue: retained fail-closed evidence keeps unsupported bare-name
  combined forms outside the accepted surface: output-first ordering,
  /dev/null input, explicit 1> output, spaced input grammar, append output,
  stderr output, unsupported command names, and arbitrary output paths.
- fixed: roadmap, Phase 12, and early POSIX docs now record the accepted
  bare-name combined closeout frontier and selected combined frontier
  checkpoint.
- deferred: arbitrary input/output paths, append in combined forms, stderr
  combined forms, pipeline-output redirection/append, persistent writable
  filesystem behavior, PATH/current-directory search, command lookup beyond
  bounded /bin, arbitrary shell grammar, live networking/SSH, Pi 5 hardware
  proof, generated-root retry, and phase transition remain outside this
  closeout.

## Evidence

- static inspection:
  tasks/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core.md.
- task-owned JSON inspection:
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/evidence-map.json.
- QEMU/substitute transcript inspection:
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/qemu-local-shell-bare-name-combined-stdin-stdout-redirection-smoke.log.
- retained regression summary inspection:
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/regression-smokes-summary.txt.
- direct combined comparison:
  tasks/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core.md
  and
  tasks/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-closeout.md.
- project doc inspection: docs/src/roadmap.md,
  docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-closeout/evidence-map.json.

## Validation

- 'jq empty
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/classification.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-closeout/evidence-map.json'
  passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed.

## Result

The accepted fixed-/bin bare-name combined stdin/stdout regular-file
redirection frontier is reconciled against retained direct combined evidence
and docs. The next mechanically objective task is
phase12-local-combined-stdin-stdout-regular-file-redirection-frontier-checkpoint-20260627.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
