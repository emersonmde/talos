# Phase 12 Local Direct Stderr Regular-File Redirection Closeout

Task: phase12-local-direct-stderr-regular-file-redirection-closeout-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Reconciled the accepted direct path-form stderr regular-file redirection core
  against the retained task record, classification, evidence map,
  QEMU/substitute transcript, regression summary, and project docs.
- Kept the accepted witness exactly '/bin/stderr 2>/tmp/stderr.txt'.
- Confirmed that child fd2 is the only descriptor rebound for the launched
  process, targets volatile-vfs:/tmp/stderr.txt, and is read back through
  descriptor-backed VFS with 'cat /tmp/stderr.txt'.
- Confirmed the later normal '/bin/stderr' witness restores stderr to
  runtime-console0/stderr and that fd0/fd1 inheritance, loader temporary
  descriptor closure, waitpid/laststatus/process-table/procfs/ps/pipestatus
  observations remain coherent.
- Selected phase12-local-bare-name-stderr-regular-file-redirection-core-20260627
  as the next mechanically objective local POSIX/VFS task because the accepted
  fixed bounded /bin lookup and direct stderr regular-file evidence make the
  bare-name witness objective.
- Did not start bare-name stderr implementation, append/truncate, arbitrary
  output path policy, pipeline-output redirection, combined input/output
  redirection, persistent writable filesystem behavior, live networking/SSH,
  Pi 5 hardware action, generated-root retry, or phase-transition work.

## Findings

- not-an-issue: the direct stderr core task record, classification JSON, and
  evidence map consistently accept only '/bin/stderr 2>/tmp/stderr.txt' and
  retain 'cat /tmp/stderr.txt' plus later '/bin/stderr' restoration as the
  readback/restoration witnesses.
- not-an-issue: retained regression evidence covers stdout regular-file
  redirection, stdin redirection, pipeline stdin redirection, command argv,
  pipeline argv, process-status VFS, zero-argument ps, pipestatus, and
  cat-banner surfaces.
- fixed: roadmap and project docs now record the direct stderr closeout and the
  selected next mechanically objective bare-name stderr task.
- deferred: bare-name stderr redirection remains implementation work for the
  selected follow-up task.
- deferred: append/truncate, arbitrary output paths, pipeline-output
  redirection, combined input/output redirection, persistent writable
  filesystem behavior, environment-backed PATH, current-directory search,
  command lookup beyond bounded /bin, arbitrary shell grammar, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition remain outside this closeout.

## Evidence

- static inspection: tasks/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core.md.
- static inspection: tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core/classification.json.
- static inspection: tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core/evidence-map.json.
- static inspection: tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core/regressions/regression-summary.txt.
- static inspection: docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and docs/src/project/early-posix-shape.md.
- task-owned evidence map: tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-closeout/evidence-map.json.

## Validation

- 'jq empty tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-closeout/evidence-map.json' passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed with the existing large
  search-index warning only.
- 'git diff --cached --check' passed.

## Result

The direct stderr regular-file redirection frontier is closed around the exact
accepted witness '/bin/stderr 2>/tmp/stderr.txt'. The next mechanically
objective queued task is
phase12-local-bare-name-stderr-regular-file-redirection-core-20260627.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
