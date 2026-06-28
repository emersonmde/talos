# Phase 12 Local Bare Name Pipeline Separated Redirection Token Core

Task id:
phase12-local-bare-name-pipeline-separated-redirection-token-core-20260628

## Summary

Implemented fixed-/bin bare-name two-stage combined pipeline support for the
exact separated redirection-token witnesses named by the task.

Accepted witnesses:

- 'stdin < /etc/banner.txt | stdin > /tmp/talos-pipeline-output-alpha.txt'
- 'stdin < /etc/banner.txt | stdin >> /tmp/talos-pipeline-output-alpha.txt'
- 'stdin < /etc/banner.txt | stderr 2> /tmp/talos-pipeline-error-beta.log'
- 'stdin < /etc/banner.txt | stderr 2>> /tmp/talos-pipeline-error-beta.log'

Both stages resolve only through bounded /bin lookup before descriptor-backed
VFS/userspace execution. The implementation preserves producer fd0 rebinding to
initramfs:/etc/banner.txt, producer fd1 pipe handoff, final-stage fd0 from the
pipe, child-only final-stage fd1/fd2 rebinding to safe volatile-vfs /tmp leaf
files, append-at-EOF semantics, descriptor-backed cat readback, lifecycle/status
and process observations, and descriptor restoration controls.

## Findings and Disposition

- fixed: Bare-name pipeline parsing now detects separated producer stdin
  redirection and separated final-stage output redirection before parsing, then
  accepts the new grammar only when both are present and the parsed request is an
  accepted combined stdin-to-stdout or stdin-to-stderr two-stage pipeline.
- fixed: Bare-name pipeline producer argument guards now accept only the exact
  separated '< /etc/banner.txt' token pair.
- fixed: Bare-name pipeline consumer argument guards and consumer parsing now
  accept only the exact separated stdout/stderr output operators with safe
  volatile /tmp leaf path operands already accepted by the bounded policy.
- fixed: QEMU/substitute local_command_loop coverage now accepts the exact
  bare-name separated pipeline witnesses and keeps mixed direct/bare stages,
  mixed separated/no-space stages, unsupported paths, reserved basenames,
  unsupported commands, consumer-only separated redirection, and separated
  explicit fd syntax fail-closed.
- fixed: Added a local command capability marker for fixed-/bin bare-name VFS
  pipeline separated redirection tokens.
- fixed: Updated roadmap and early POSIX shape docs with the accepted/deferred
  pipeline grammar frontier.
- deferred: Separated explicit fd syntax such as '1 > path' or '2 > path',
  mixed direct/bare broadening, multistage separated-token pipelines,
  PATH/current-directory lookup, command lookup beyond bounded /bin, arbitrary
  shell grammar, persistence, generated-root retry, boot publication, live
  networking/SSH, Pi 5 hardware proof, and phase transition.
- not-an-issue: Successful command-visible behavior remains backed by
  descriptor/VFS/userspace layers; no fake/kernel-backed command expansion was
  added.

## Evidence

- static inspection: src/local_command_loop.rs extends only the fixed-/bin
  bare-name pipeline parsing and argument guards, with a post-parse gate that
  requires both separated producer stdin and separated final-stage output
  redirection before accepting the grammar.
- fmt/lint: cargo fmt --all -- --check: passed.
- QEMU/substitute unit tests: cargo -Zjson-target-spec test --quiet
  local_command_loop: passed; retained in
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-separated-redirection-token-core/qemu-substitute-local-command-loop.log.
- evidence validation: jq empty
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-separated-redirection-token-core/classification.json
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-separated-redirection-token-core/evidence-map.json:
  passed.
- diff validation: git diff --check: passed.
- docs validation: /home/node/.cargo/bin/mdbook build: passed; search index size
  warning retained.
- staged diff validation: git diff --cached --check: passed before commit.

## Accepted Frontier

Accepted fixed-/bin bare-name pipeline separated redirection tokens are limited
to:

    stdin < /etc/banner.txt | stdin > /tmp/talos-pipeline-output-alpha.txt
    stdin < /etc/banner.txt | stdin >> /tmp/talos-pipeline-output-alpha.txt
    stdin < /etc/banner.txt | stderr 2> /tmp/talos-pipeline-error-beta.log
    stdin < /etc/banner.txt | stderr 2>> /tmp/talos-pipeline-error-beta.log

Mixed direct/bare stages, mixed no-space/separated stages, consumer-only
separated pipeline redirection, unsupported paths, reserved cross-stream
basenames, missing command names, and separated explicit fd syntax remain
rejected before file creation/write or new successful process records.

selected_next_task=phase12-local-separated-redirection-token-frontier-checkpoint-20260628.

## Scope Statement

No Pi 5 hardware/lab action, boot publication, generated-root retry, live
networking/SSH, persistence, arbitrary paths, separated explicit fd syntax, or
phase transition was performed or claimed.

Commit: recorded in talos-supervisor-state.json acceptance evidence.
