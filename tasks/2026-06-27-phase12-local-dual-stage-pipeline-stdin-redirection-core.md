# Phase 12 Local Dual-Stage Pipeline Stdin Redirection Core

Task: \`phase12-local-dual-stage-pipeline-stdin-redirection-core-20260627\`

Status: accepted pending commit.

## Scope

- Accepted exactly \`/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt\`.
- Accepted exactly \`stdin </etc/banner.txt | stdin </etc/banner.txt\` through the already accepted fixed bounded \`/bin\` lookup.
- Kept the implementation local-only: no live networking, SSH, Pi 5 hardware action, generated-root retry, writable filesystem behavior, output regular-file redirection, or phase transition.

## Findings

- fixed: the pipeline consumer-stdin-redirection eligibility had required producer stdin to be absent. It now permits only \`None\` or \`StdinFromEtcBanner\` on the \`/bin/stdin\` producer while preserving all existing path, argc, and output-redirection guards.
- fixed: the shell-visible pipe source now distinguishes dual-stage stdin redirection as \`shell-pipe-dual-stdin-redirection-from-file\`.
- fixed: direct and bare QEMU/substitute smoke expectations now treat command 16 as the accepted dual-stage witness, command 17 as the mixed direct/bare fail-closed witness, and assert the pipe summary with \`bytes-written=0x3d\`, \`bytes-read=0x0\`, \`reader-eof=false\`, and restored shell state.
- fixed: unit coverage now uses deterministic dual-stage scripted-input fixtures so both child fd0 entries are regular-file descriptors rather than depending on runtime-console input availability.
- not-an-issue: the retained consumer-stage-only QEMU/substitute smokes still prove \`/bin/stdin | /bin/stdin </etc/banner.txt\` and \`stdin | stdin </etc/banner.txt\`; they remain regression surfaces rather than the unit fixture's first command.
- fixed: mixed direct/bare dual-stage forms fail closed without adding successful process records.
- deferred: broader redirection placement, output redirection, append/truncate, writable filesystem behavior, multistage pipelines, concurrent pipeline scheduling, PATH/current-directory search, and arbitrary shell grammar remain outside this task.

## Evidence

- static inspection: \`src/local_command_loop.rs\`, \`src/target/qemu_virt.rs\`, and \`scripts/qemu-local-serial-command-loop-smoke.sh\`.
- fmt/lint/typecheck: \`cargo fmt --all -- --check\`.
- unit tests: \`cargo -Zjson-target-spec test --quiet local_command_loop\` passed with 851 tests.
- QEMU/substitute smoke: \`tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-core/qemu-local-shell-direct-pipeline-consumer-stdin-redirection-smoke.log\` ends PASS and records the direct dual-stage witness plus the mixed direct-producer/bare-consumer fail-closed witness.
- QEMU/substitute smoke: \`tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-core/qemu-local-shell-bare-name-pipeline-consumer-stdin-redirection-smoke.log\` ends PASS and records the bare-name dual-stage witness plus the mixed bare-producer/direct-consumer fail-closed witness.
- QEMU/substitute regressions: \`tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-core/regressions/\` contains PASS transcripts for command stdin redirection, producer-stage pipeline stdin redirection, consumer-stage pipeline stdin redirection, direct/bare command argv, direct/bare pipeline argv, process-status VFS, zero-argument ps, pipestatus, and cat-banner.

## Validation

- \`cargo fmt --all -- --check\` passed.
- \`cargo -Zjson-target-spec test --quiet local_command_loop\` passed.
- Direct dual-stage QEMU/substitute smoke passed.
- Bare-name dual-stage QEMU/substitute smoke passed.
- Retained QEMU/substitute regression smokes passed.
- \`jq empty tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-core/evidence-map.json\` passed.
- \`git diff --check\` passed.
- \`/home/node/.cargo/bin/mdbook build\` passed.
- \`git diff --cached --check\` passed.

## Result

The accepted core composes the existing descriptor-backed VFS/open/read, loader, userspace launch/status, descriptor, process-table, pipe, and read-only stdin redirection layers. Each stage receives its own initramfs-backed fd0. The producer writes the redirected banner output to the serialized pipe surface; the consumer reads its own redirected file to EOF. Unsupported variants fail closed without adding successful process records.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
