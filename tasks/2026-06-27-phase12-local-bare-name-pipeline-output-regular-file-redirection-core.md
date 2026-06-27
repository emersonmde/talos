# Phase 12 Local Bare-Name Pipeline-Output Regular-File Redirection Core

Task id: phase12-local-bare-name-pipeline-output-regular-file-redirection-core-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest fixed-/bin bare-name pipeline-output redirection path:

    stdout | stdin >/tmp/pipeline-report.txt

Both pipeline stages resolve only through the bounded /bin lookup to
'/bin/stdout' and '/bin/stdin', load through descriptor-backed VFS open/read,
use the accepted userspace launch/status path, and hand off data through the
accepted pipe endpoint. Only the final stage receives child-only fd1
redirection to 'volatile-vfs:/tmp/pipeline-report.txt'; the shell fd1 is
restored afterward.

## Non-Goals

This task does not implement append pipeline-output redirection, stderr forms,
input or combined pipeline redirections, arbitrary output paths, persistent
writable filesystem behavior, environment-backed PATH, current-directory search,
command lookup beyond bounded /bin, arbitrary shell grammar, unbounded or
concurrent pipelines, live networking/SSH, Pi 5 hardware proof, generated-root
retry, or a phase transition.

## Findings

- fixed: Bare-name two-stage pipeline parsing now routes the consumer stage
  through a pipeline-consumer parser that accepts exactly
  'stdin >/tmp/pipeline-report.txt' as the final stage output sink.
- fixed: The accepted witness records '/bin/stdout' and '/bin/stdin'
  VFS open/read execution, minimal argv0 startup records for both resolved
  paths, pipe handoff, child-only fd1 redirection to
  'volatile-vfs:/tmp/pipeline-report.txt', descriptor-backed readback, and
  coherent 'waitpid', 'laststatus', '/proc/talos/processes', 'ps', and
  'pipestatus'.
- fixed: Fail-closed coverage rejects alternate output targets, append syntax,
  wrong final-stage program, explicit '1>', spaced output grammar, and consumer
  names containing path separators.
- not-an-issue: The QEMU-substitute cargo test harness runs the whole no_std
  suite even when a focused test filter is supplied; the retained transcript
  therefore records the focused witness under a broader '864 passed' test run.
- deferred: Append pipeline-output forms, stderr forms, input/combined pipeline
  redirections, arbitrary paths, persistence, live network/SSH, Pi 5 hardware
  proof, generated-root retry, and phase transition remain out of scope.

## Evidence

- static inspection:
  - 'src/local_command_loop.rs' parser and regression diff.
  - 'docs/src/roadmap.md'.
  - 'docs/src/project/phase12-networking-ssh.md'.
  - 'docs/src/project/early-posix-shape.md'.
- QEMU/substitute focused transcript:
  'tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/qemu-substitute-focused-test.log'.
- classification:
  'tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/classification.json'.
- evidence map:
  'tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/evidence-map.json'.

## Validation

- 'cargo -Zjson-target-spec test --quiet local_command_loop_redirects_bare_name_pipeline_consumer_stdout_to_volatile_regular_file': passed; retained transcript reports 'test result: ok. 864 passed'.
- 'cargo fmt --all -- --check': passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop': passed; QEMU-substitute harness reported 'test result: ok. 864 passed'.
- 'jq empty tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/evidence-map.json': passed.
- 'git diff --check': passed.
- '/home/node/.cargo/bin/mdbook build': passed; HTML backend wrote book with existing large search-index warning.
- 'git diff --cached --check': passed before commit.

## Result

Accepted boundary: 'stdout | stdin >/tmp/pipeline-report.txt' succeeds only
through fixed-/bin resolution to '/bin/stdout' and '/bin/stdin'. The result
file is volatile only and read back through descriptor-backed
'cat /tmp/pipeline-report.txt'.

No live network/SSH work, Pi 5 hardware action, generated-root retry, append or
stderr pipeline-output forms, arbitrary paths, persistence claim, or phase
transition was performed.

selected_next_task: phase12-local-bare-name-pipeline-output-regular-file-redirection-closeout-20260627.
