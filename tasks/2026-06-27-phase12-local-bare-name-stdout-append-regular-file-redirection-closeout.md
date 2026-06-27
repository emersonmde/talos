# Phase 12 Local Bare-Name Stdout Append Regular-File Redirection Closeout

Task: phase12-local-bare-name-stdout-append-regular-file-redirection-closeout-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Reviewed the accepted direct and fixed-/bin bare-name stdout append
  regular-file redirection core tasks, classifications, evidence maps,
  QEMU/substitute transcripts, regression summaries, and project docs.
- Reconciled the accepted bare-name witness sequence
  'stdout >/tmp/stdout.txt' followed by 'stdout >>/tmp/stdout.txt' against the
  direct path-form append evidence and retained descriptor-backed volatile VFS
  regular-file output/readback evidence.
- Kept the accepted surface limited to direct path-form and fixed-/bin
  bare-name stdout append into '/tmp/stdout.txt'. The next mechanically
  objective same-lane task is the already queued stdout append frontier
  checkpoint because direct and bare-name append evidence are accepted and
  committed.
- Did not start stderr append, arbitrary output paths, pipeline-output append,
  combined input/output redirection, persistent writable filesystem behavior,
  live networking/SSH, Pi 5 hardware action, generated-root retry, runtime
  feature implementation, or phase transition work.

## Findings

- not-an-issue: the bare-name core task's accepted witness sequence is exactly
  'stdout >/tmp/stdout.txt' then 'stdout >>/tmp/stdout.txt'; retained evidence
  records fixed bounded /bin lookup to '/bin/stdout', child-only fd1 rebinding
  to 'volatile-vfs:/tmp/stdout.txt', initial op=sink/truncate, second op=append
  at regular-file EOF, and descriptor-backed 'cat /tmp/stdout.txt' readback of
  two stdout fixture writes in order.
- not-an-issue: retained evidence records later normal 'stdout' routing fd1
  back to runtime-console0/stdout and coherent waitpid, laststatus, process
  table, /proc/talos/processes, zero-argument ps, and pipestatus-compatible
  observations.
- not-an-issue: retained fail-closed evidence keeps unsupported bare-name and
  direct forms outside the accepted surface, including arbitrary append paths,
  stderr append, pipeline-output append, combined redirection,
  PATH/current-directory lookup, and command lookup beyond bounded /bin.
- fixed: roadmap, Phase 12, and early POSIX docs now record the accepted
  bare-name stdout append closeout frontier and selected stdout append frontier
  checkpoint.
- deferred: stderr append, arbitrary output path policy, pipeline-output
  append, combined input/output redirection, kernel-backed command redirection,
  persistent writable filesystem behavior, PATH/current-directory search,
  command lookup beyond bounded /bin, arbitrary shell grammar, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition remain outside this closeout.

## Evidence

- static inspection:
  tasks/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core.md,
  tasks/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-closeout.md,
  and
  tasks/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core.md.
- task-owned JSON inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core/classification.json,
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core/classification.json,
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core/evidence-map.json.
- QEMU/substitute transcript inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core/qemu-local-shell-direct-stdout-regular-file-append-redirection-smoke.log
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core/qemu-local-shell-bare-name-stdout-regular-file-append-redirection-smoke.log.
- project doc inspection: docs/src/roadmap.md,
  docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-closeout/evidence-map.json.

## Validation

- 'jq empty
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core/classification.json
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core/classification.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-closeout/evidence-map.json'
  passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed.

## Result

The accepted bare-name stdout append regular-file redirection frontier is
reconciled against retained direct and bare-name evidence and docs. The next
mechanically objective task is
phase12-local-stdout-append-regular-file-redirection-frontier-checkpoint-20260627.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
