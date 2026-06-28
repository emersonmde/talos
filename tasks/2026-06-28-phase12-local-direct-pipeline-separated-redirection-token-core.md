# Phase 12 Local Direct Pipeline Separated Redirection Token Core

Task id:
phase12-local-direct-pipeline-separated-redirection-token-core-20260628

## Summary

Implemented direct absolute-path two-stage combined pipeline support for the
exact separated redirection-token witnesses named by the task.

Accepted witnesses:

- '/bin/stdin < /etc/banner.txt | /bin/stdin > /tmp/talos-pipeline-output-alpha.txt'
- '/bin/stdin < /etc/banner.txt | /bin/stdin >> /tmp/talos-pipeline-output-alpha.txt'
- '/bin/stdin < /etc/banner.txt | /bin/stderr 2> /tmp/talos-pipeline-error-beta.log'
- '/bin/stdin < /etc/banner.txt | /bin/stderr 2>> /tmp/talos-pipeline-error-beta.log'

The implementation remains descriptor-backed: VFS executable open/read,
userspace launch/status, producer fd0 rebinding to initramfs:/etc/banner.txt,
pipe handoff, final-stage fd1/fd2 rebinding to safe volatile-vfs /tmp leaf
files, append-at-EOF semantics, descriptor-backed cat readback, and descriptor
restoration controls.

## Findings and Disposition

- fixed: Direct absolute-path pipeline parsing now accepts separated '<',
  '>', '>>', '2>', and '2>>' operator tokens only when the parsed request is an
  accepted combined stdin-to-stdout or stdin-to-stderr two-stage pipeline.
- fixed: Final-stage separated stdout/stderr path operands reuse the accepted
  volatile /tmp leaf policy and preserve append/truncate behavior.
- fixed: QEMU/substitute local_command_loop coverage now treats the direct
  separated pipeline witnesses as accepted and keeps consumer-only separated
  redirection, mixed direct/bare stages, unsupported paths, reserved basenames,
  unsupported commands, and separated explicit fd syntax fail-closed.
- fixed: Added a local command capability marker for direct absolute-path VFS
  pipeline separated redirection tokens.
- fixed: Updated roadmap and early POSIX shape docs with the accepted/deferred
  pipeline grammar frontier.
- deferred: Fixed-/bin bare-name pipeline separated-token support, mixed
  direct/bare broadening, multistage separated-token pipelines, separated
  explicit fd syntax such as '1 > path' or '2 > path',
  PATH/current-directory lookup, command lookup beyond bounded /bin, arbitrary
  shell grammar, persistence, generated-root retry, boot publication, live
  networking/SSH, Pi 5 hardware proof, and phase transition.
- not-an-issue: Successful command-visible behavior remains backed by
  descriptor/VFS/userspace layers; no fake/kernel-backed command expansion was
  added.

## Evidence

- static inspection: src/local_command_loop.rs separates absolute-path pipeline
  bounded checks from bare-name pipeline parsing and requires both separated
  producer stdin redirection and separated final-stage output redirection before
  accepting the new grammar.
- fmt/lint: cargo fmt --all -- --check: passed.
- QEMU/substitute unit tests: cargo -Zjson-target-spec test --quiet
  local_command_loop: passed; retained in
  tasks/evidence/2026-06-28-phase12-local-direct-pipeline-separated-redirection-token-core/qemu-substitute-local-command-loop.log.
- evidence validation: jq empty
  tasks/evidence/2026-06-28-phase12-local-direct-pipeline-separated-redirection-token-core/classification.json
  tasks/evidence/2026-06-28-phase12-local-direct-pipeline-separated-redirection-token-core/evidence-map.json:
  passed.
- diff validation: git diff --check: passed.
- docs validation: /home/node/.cargo/bin/mdbook build: passed.
- staged diff validation: git diff --cached --check: passed before commit.

## Accepted Frontier

Accepted direct path-form pipeline separated redirection tokens are limited to:

    /bin/stdin < /etc/banner.txt | /bin/stdin > /tmp/talos-pipeline-output-alpha.txt
    /bin/stdin < /etc/banner.txt | /bin/stdin >> /tmp/talos-pipeline-output-alpha.txt
    /bin/stdin < /etc/banner.txt | /bin/stderr 2> /tmp/talos-pipeline-error-beta.log
    /bin/stdin < /etc/banner.txt | /bin/stderr 2>> /tmp/talos-pipeline-error-beta.log

Consumer-only separated pipeline redirection remains rejected, so the new
surface does not broaden the already accepted stdout-pipe and stderr-pipe
controls. Fixed-/bin bare-name pipeline separated tokens are deferred to the
queued successor.

selected_next_task=phase12-local-bare-name-pipeline-separated-redirection-token-core-20260628.

## Scope Statement

No Pi 5 hardware/lab action, boot publication, generated-root retry, live
networking/SSH, persistence, arbitrary paths, bare-name pipeline
separated-token support, separated explicit fd syntax, or phase transition was
performed or claimed.
