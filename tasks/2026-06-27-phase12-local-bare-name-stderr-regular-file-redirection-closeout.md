# Phase 12 Local Bare-Name Stderr Regular-File Redirection Closeout

Task: phase12-local-bare-name-stderr-regular-file-redirection-closeout-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Reviewed the accepted bare-name stderr regular-file redirection core task, classification, evidence map, QEMU/substitute transcript, regression summary, and project docs.
- Reconciled the accepted witness 'stderr 2>/tmp/stderr.txt' against retained direct stderr, stdout regular-file redirection, bounded /bin lookup, descriptor restoration, process-table/status, /proc/talos/processes, ps, and pipestatus evidence.
- Selected phase12-local-stderr-regular-file-redirection-frontier-checkpoint-20260627 as the next mechanically objective same-lane task because direct and bare-name stderr regular-file evidence is accepted and committed.
- Did not start append/truncate, arbitrary output path policy, pipeline-output redirection, combined input/output redirection, persistent writable filesystem behavior, live networking/SSH, Pi 5 hardware action, generated-root retry, or phase transition work.

## Findings

- not-an-issue: the core task's accepted witness is exactly 'stderr 2>/tmp/stderr.txt', and the retained transcript records bounded /bin resolution to '/bin/stderr', child-only fd2 rebinding to volatile-vfs:/tmp/stderr.txt, descriptor-backed 'cat /tmp/stderr.txt' readback, and later normal 'stderr' restoration to runtime-console0/stderr.
- not-an-issue: retained regression evidence covers direct stderr, stdout regular-file redirection, stdin redirection, pipeline stdin redirection, command argv, pipeline argv, process-status VFS/ps, pipestatus, and cat-banner surfaces.
- fixed: roadmap, Phase 12, and early POSIX docs now record the accepted bare-name stderr closeout frontier and the selected checkpoint task.
- deferred: append/truncate, arbitrary output paths, pipeline-output redirection, combined input/output redirection, persistent writable filesystem behavior, PATH/current-directory search, command lookup beyond bounded /bin, arbitrary shell grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase transition remain outside this closeout.

## Evidence

- static inspection: tasks/2026-06-27-phase12-local-bare-name-stderr-regular-file-redirection-core.md.
- task-owned JSON inspection: tasks/evidence/2026-06-27-phase12-local-bare-name-stderr-regular-file-redirection-core/classification.json and evidence-map.json.
- QEMU/substitute transcript inspection: tasks/evidence/2026-06-27-phase12-local-bare-name-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log.
- QEMU/substitute regression inspection: tasks/evidence/2026-06-27-phase12-local-bare-name-stderr-regular-file-redirection-core/regressions/regression-summary.txt.
- closeout evidence map: tasks/evidence/2026-06-27-phase12-local-bare-name-stderr-regular-file-redirection-closeout/evidence-map.json.

## Validation

- 'jq empty tasks/evidence/2026-06-27-phase12-local-bare-name-stderr-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-bare-name-stderr-regular-file-redirection-core/evidence-map.json tasks/evidence/2026-06-27-phase12-local-bare-name-stderr-regular-file-redirection-closeout/evidence-map.json' passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed with the existing large search-index warning only.
- 'git diff --cached --check' passed.

## Result

The accepted bare-name stderr regular-file redirection frontier is reconciled against retained evidence and docs. The next mechanically objective task is phase12-local-stderr-regular-file-redirection-frontier-checkpoint-20260627.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
