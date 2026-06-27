# Phase 12 Local Combined Stdin Stdout Regular-File Redirection Frontier Checkpoint

Task: phase12-local-combined-stdin-stdout-regular-file-redirection-frontier-checkpoint-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Reviewed the accepted direct and fixed-/bin bare-name combined stdin/stdout
  regular-file redirection tasks, closeouts, classifications, evidence maps,
  QEMU/substitute transcripts, retained regression summary, and project docs.
- Reconciled the exact accepted direct witness
  '/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt' and fixed-/bin
  bare-name witness 'stdin </etc/banner.txt >/tmp/stdin-report.txt' against
  retained stdin redirection, stdout regular-file redirection, append frontier,
  process-status, ps, pipestatus, and cat-banner controls.
- Kept the accepted surface limited to the two exact combined stdin/stdout
  witnesses. There is no later queued same-lane local POSIX/VFS task after this
  checkpoint, so supervisor planning is required before another worker
  promotion.
- Did not implement runtime feature changes, arbitrary input/output paths,
  append in combined forms, stderr combined forms, pipeline-output redirection
  or append, persistent writable filesystem behavior, live networking/SSH,
  Pi 5 hardware action, generated-root retry, or a phase transition.

## Findings

- not-an-issue: the accepted direct witness is exactly
  '/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt'. Retained evidence
  records descriptor-backed VFS launch of '/bin/stdin', child-only fd0 from
  initramfs:/etc/banner.txt, child-only fd1 to
  volatile-vfs:/tmp/stdin-report.txt, fd2 inherited as stdio output,
  shell-restored=true, closed loader temporary descriptors, userspace report
  write through redirected fd1, and descriptor-backed
  'cat /tmp/stdin-report.txt' readback.
- not-an-issue: the accepted fixed-/bin bare-name witness is exactly
  'stdin </etc/banner.txt >/tmp/stdin-report.txt'. Retained evidence records
  bounded resolution only to '/bin/stdin' before using the same child-only fd0
  and fd1 descriptor mutations as the direct witness.
- not-an-issue: retained fail-closed evidence keeps unsupported direct and
  bare-name combined forms outside the accepted surface: output-first ordering,
  spaced input grammar, /dev/null input, explicit 1> output, append output,
  stderr output, unsupported command names, and arbitrary output paths.
- fixed: roadmap, Phase 12, and early POSIX docs now record the combined
  stdin/stdout frontier checkpoint and that no later queued same-lane
  local POSIX/VFS task is mechanically objective.
- deferred: arbitrary input/output paths, append in combined forms, stderr
  combined forms, pipeline-output redirection and append, persistent writable
  filesystem behavior, environment-backed PATH, current-directory search,
  command lookup beyond bounded /bin, arbitrary shell grammar, unbounded or
  concurrent pipelines, scheduler concurrency, fork/signals, process
  groups/sessions, live networking/SSH, Pi 5 hardware proof, generated-root
  retry, and phase transition remain outside this checkpoint.

## Evidence

- static inspection:
  tasks/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core.md,
  tasks/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-closeout.md,
  tasks/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core.md,
  and
  tasks/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-closeout.md.
- task-owned JSON inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/classification.json,
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/classification.json,
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/evidence-map.json.
- QEMU/substitute transcript inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/qemu-local-shell-direct-combined-stdin-stdout-redirection-smoke.log
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/qemu-local-shell-bare-name-combined-stdin-stdout-redirection-smoke.log.
- retained regression summary inspection:
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/regression-smokes-summary.txt.
- project doc inspection: docs/src/roadmap.md,
  docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- checkpoint evidence map:
  tasks/evidence/2026-06-27-phase12-local-combined-stdin-stdout-regular-file-redirection-frontier-checkpoint/evidence-map.json.

## Validation

- 'jq empty
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/classification.json
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/classification.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-combined-stdin-stdout-regular-file-redirection-frontier-checkpoint/evidence-map.json'
  passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed.

## Result

The accepted combined stdin/stdout regular-file redirection frontier is limited
to:

~~~text
/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt
stdin </etc/banner.txt >/tmp/stdin-report.txt
~~~

The direct form and fixed-/bin bare-name form both use descriptor-backed VFS
program loading, child-only fd0 from initramfs:/etc/banner.txt, child-only fd1
to volatile-vfs:/tmp/stdin-report.txt, stdio stderr inheritance, userspace
report writing through redirected fd1, descriptor-backed readback, and shell
descriptor restoration. Unsupported direct and bare-name combined forms remain
fail-closed.

selected_next_task: null.
planningNeeded: true because no later queued same-lane local POSIX/VFS task
has complete objective dependencies, acceptance criteria, validation gates,
docs requirements, and evidence requirements.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
