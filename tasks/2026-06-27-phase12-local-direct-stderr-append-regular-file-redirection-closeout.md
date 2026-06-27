# Phase 12 Local Direct Stderr Append Regular-File Redirection Closeout

Task: phase12-local-direct-stderr-append-regular-file-redirection-closeout-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Reviewed the accepted direct stderr append regular-file redirection core task,
  classification, evidence map, QEMU/substitute transcript, regression
  summaries, and project docs.
- Reconciled the accepted witness sequence
  '/bin/stderr 2>/tmp/stderr.txt' followed by
  '/bin/stderr 2>>/tmp/stderr.txt' against retained descriptor-backed volatile
  VFS regular-file stderr output/readback evidence.
- Kept the accepted surface limited to direct path-form stderr append into
  '/tmp/stderr.txt'. The next mechanically objective same-lane task is the
  already queued bare-name stderr append core because direct stderr append
  evidence is accepted and committed.
- Did not start bare-name stderr append implementation, arbitrary output paths,
  pipeline-output append, combined input/output redirection, persistent writable
  filesystem behavior, live networking/SSH, Pi 5 hardware action,
  generated-root retry, or phase transition work.

## Findings

- not-an-issue: the core task's accepted witness sequence is exactly
  '/bin/stderr 2>/tmp/stderr.txt' then
  '/bin/stderr 2>>/tmp/stderr.txt'; retained evidence records child-only fd2
  rebinding to 'volatile-vfs:/tmp/stderr.txt', initial op=sink/truncate, second
  op=append at regular-file EOF, and descriptor-backed
  'cat /tmp/stderr.txt' readback of two stderr fixture writes in order.
- not-an-issue: retained evidence records later normal '/bin/stderr' routing
  fd2 back to runtime-console0/stderr and coherent waitpid, laststatus,
  process table, /proc/talos/processes, zero-argument ps, and
  pipestatus-compatible observations.
- not-an-issue: retained fail-closed evidence keeps unsupported direct and
  bare-name forms outside the accepted surface, including arbitrary stderr
  append paths, bare-name stderr append, pipeline-output append, combined
  redirection, PATH/current-directory lookup, and command lookup beyond bounded
  /bin.
- fixed: roadmap, Phase 12, and early POSIX docs now record the accepted direct
  stderr append closeout frontier and selected bare-name stderr append core.
- deferred: bare-name stderr append remains separate implementation work.
  Arbitrary output path policy, pipeline-output append, combined input/output
  redirection, kernel-backed command redirection, persistent writable filesystem
  behavior, PATH/current-directory search, command lookup beyond bounded /bin,
  arbitrary shell grammar, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition remain outside this closeout.

## Evidence

- static inspection:
  tasks/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-core.md.
- task-owned JSON inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-core/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-core/evidence-map.json.
- QEMU/substitute transcript inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-core/qemu-local-shell-direct-stderr-regular-file-append-redirection-smoke.log.
- project doc inspection: docs/src/roadmap.md,
  docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-closeout/evidence-map.json.

## Validation

- 'jq empty
  tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-core/classification.json
  tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-core/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-closeout/evidence-map.json'
  passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed.

## Result

The accepted direct stderr append regular-file redirection frontier is
reconciled against retained evidence and docs. The next mechanically objective
task is
phase12-local-bare-name-stderr-append-regular-file-redirection-core-20260627.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
