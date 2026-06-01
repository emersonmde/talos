# Phase 10 Pi 5 Local Ls Bin Proof Task

Task: phase10-pi5-local-ls-bin-proof-20260601

Status: blocked-capture-gap

## Goal

Carry the accepted bounded `ls /bin` directory-listing command to serialized
Raspberry Pi 5 serial hardware evidence.

## Scope

This task added only the Pi 5 proof scenario and staging scripts needed to boot
the accepted local `ls /bin` command on hardware:

- build.rs
- src/target/rpi5.rs
- scripts/rpi5-local-ls-bin-image.sh
- scripts/rpi5-local-ls-bin-boot-tree.sh

The local command-loop semantics did not change. The proof scenario expects
exact `ls /bin` dispatch, descriptor-backed fd0/stdout, one response line from
the accepted read-only initramfs fixture, and final
`pi5-local-ls-bin-complete` / `rpi5-local-ls-bin-proof: PASS` output.

## Evidence

Evidence root:

- tasks/evidence/2026-06-01-pi5-local-ls-bin-proof/

Local gates and candidate identity:

- local1-candidate/local-gates-summary.txt
- local1-candidate/qemu-local-ls-bin-smoke.log
- local1-candidate/archive-review.txt
- local1-candidate/archive-sha256.txt
- local1-candidate/archive-kernel-sha256.txt
- local1-candidate/boot-tree-files.txt

Initial candidate run:

- local1-candidate/proof-result.txt
- local1-candidate/serial-full-window-after-write.clean.txt
- local1-candidate/tftp-delta-before-restore.json
- local1-candidate/post-restore-status.json

Known-good control after the inconclusive candidate run:

- local1-known-good-ls-root-control/control-result.txt
- local1-known-good-ls-root-control/serial-combined.clean.txt
- local1-known-good-ls-root-control/tftp-delta-before-restore.json
- local1-known-good-ls-root-control/post-restore-status.json

Unchanged candidate rerun:

- local2-unchanged-candidate-rerun/proof-result.txt
- local2-unchanged-candidate-rerun/serial-combined.clean.txt
- local2-unchanged-candidate-rerun/tftp-delta-before-restore.json
- local2-unchanged-candidate-rerun/post-restore-status.json

Final restore:

- final-restore-snapshot.json
- final-post-restore-status.json

## Result

The initial candidate and unchanged candidate rerun both proved fresh staging
and command-loop completion but did not satisfy the visible-output acceptance
gate:

~~~text
prompt_found=1
input_found=0
init_found=0
fd_marker_found=1
stdout_marker_found=1
ready_next_found=1
classification_found=1
pass_found=1
tftp_fetch_found=1
restore_found=1
~~~

The retained serial window shows a fresh `rpi5-local-ls-bin-proof: ready
command=0` prompt, descriptor-backed input/output markers, the post-dispatch
suffix with `raw-bytes=8`, complete `ready-for-next prompt=true`, final
`classification=pi5-local-ls-bin-complete`, and
`rpi5-local-ls-bin-proof: PASS`. It does not retain the visible typed
`ls /bin` input or the `init` output line required by the task acceptance
criteria.

The known-good `ls /` control passed under the same lab path after the first
inconclusive candidate run. It captured fresh prompt/PASS/TFTP/restore evidence
and visible root entries, but its retained stream also dropped the earliest
post-write prefix. That supports classifying the `ls /bin` miss as a serial
capture-window/output retention gap rather than a candidate boot or command
dispatch failure.

No visibility replay for `init` was added after the inconclusive runs because
the task non-goals forbid adding visibility shims or changing runtime behavior
inside this proof task. Supervisor planning is required to decide whether the
next bounded task should authorize a proof-harness replay for the already
observed `response_lines=1` path, change the capture method/acceptance gate, or
choose another feature-led step.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 350 tests
  and existing dead-code warnings for `DescriptorBackedLocalCommandSink`.
- QEMU/substitute: `scripts/qemu-local-ls-bin-smoke.sh --quiet` passed.
- image/archive inspection: `scripts/rpi5-archive-review.sh
  target/talos-rpi5-local-ls-bin-local1.tar.gz` passed.
- lab-controller API: candidate and control archives published, TFTP fetched
  `kernel_2712.img`, and each run restored the pre-run tree hash
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- serial hardware boot/output: candidate reached prompt, descriptor markers,
  ready-next, final classification, and PASS; visible `ls /bin` and `init`
  were not retained, so the hardware proof is not accepted.

## Deferred Surfaces

This task does not accept broader path traversal, recursive/general listing,
relative paths, `cd`, file reads, writable filesystem state,
descriptor-backed filesystem syscalls, userspace execution, process lifecycle,
terminal/session behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
or DMA/cache-driver policy.

## Hardware Lock

hardwareTestLock was acquired for
phase10-pi5-local-ls-bin-proof-20260601, the pre-run boot tree was restored
after each hardware run, and the final lab status returned to the pre-run tree
hash. The lock was released after blocked evidence capture with the lab restored.
