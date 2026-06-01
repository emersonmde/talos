# Phase 10 Pi 5 Local Ls Root Capture-Window Proof Task

Task: phase10-pi5-local-ls-root-capture-window-proof-20260601

Status: accepted

## Goal

Run one unchanged Raspberry Pi 5 'ls /' candidate proof with a retained serial
window that begins before the prompt/write boundary and captures the full
response through visible root entries and next-prompt readiness.

## Scope

This task did not change Talos runtime code, proof harness code, boot scripts,
lab-controller code, or acceptance criteria. It reused the existing accepted
local 'ls /' candidate archive from local4:

- target/talos-rpi5-local-ls-root-local1.tar.gz
- archive sha256:
  16f5a053e05459239645b96eace01ee7f46139fa558b264af8a336a02d2a112c
- kernel sha256:
  904eeb9348ff1c0d1ade43c8e441b68f3bf9cef01055b2af2c3ad7d23a82eb24

The run serialized hardware access through hardwareTestLock, published the
unchanged candidate archive, power-cycled the Pi 5, observed from serial cursor
3900300, wrote 'ls /', retained the full post-power serial window through
cursor 3911477, captured TFTP evidence before restore, and restored the
pre-run boot tree.

## Evidence

Evidence directory:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local5-capture-window-proof/

Local pre-publication gates:

- local-gates-summary.txt
- qemu-local-ls-root-smoke.log

Archive and candidate identity:

- archive-review.txt
- archive-sha256.txt
- archive-kernel-sha256.txt
- post-publish-status.json
- post-publish-boot-files.json

Hardware proof:

- proof-result.txt
- serial-full-window-after-write.txt
- serial-full-window-after-write.clean.txt
- serial-full-window-after-write.json
- serial-prompt-window.txt
- serial-write-ls-root-request.json
- serial-write-ls-root-response.json
- tftp-delta-before-restore.json

Restore proof:

- pre-run-snapshot.json
- restore-snapshot.json
- post-restore-status.json

## Result

proof-result.txt classifies the run as accepted-proof:

~~~text
prompt_found=1
input_found=1
bin_found=1
dir_found=1
empty_found=1
etc_found=1
fd_marker_found=1
stdout_marker_found=1
ready_next_found=1
classification_found=1
pass_found=1
~~~

The retained serial window shows descriptor-backed fd0/stdout markers, the
'rpi5-local-ls-root-proof: ready command=0' prompt, input='ls /', visible
root entries:

~~~text
bin
dir
empty
etc
~~~

and the complete next prompt and final proof lines:

~~~text
rpi5-local-ls-root-proof: ready-for-next prompt=true
rpi5-local-ls-root-proof: final participants=1 expected=1 errors=0 classification=pi5-local-ls-root-complete
rpi5-local-ls-root-proof: PASS
~~~

TFTP evidence before restore recorded served candidate boot requests including
da591740/kernel_2712.img. The candidate boot tree hash was
697809f9570d28254de04a56fa7b45173e8dfbd06c12fb81b2dfbb2944adae25.
Post-restore status returned to the pre-run tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with existing
  dead-code warnings for DescriptorBackedLocalCommandSink.
- QEMU/substitute: scripts/qemu-local-ls-root-smoke.sh --quiet passed.
- QEMU/substitute regressions: echo, pwd, literal echo, line editing, Ctrl-C
  line cancel, Ctrl-U line kill, help, and serial-write ingress control passed
  in local-gates-summary.txt.
- image/archive inspection: scripts/rpi5-archive-review.sh
  target/talos-rpi5-local-ls-root-local1.tar.gz passed.
- serial hardware boot/output: local5 capture-window proof passed.
- lab-controller API: post-proof restore returned the pre-run tree hash.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged static inspection: git diff --cached --check passed before commit.

## Deferred Surfaces

This accepts only the bounded kernel-backed, prompt-local 'ls /' hardware
frontier. Broad shell parsing, recursive listing, path traversal, writable
filesystem state, descriptor-backed filesystem syscalls, userspace shell
execution, process lifecycle, terminal/session behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
deferred.

## Hardware Lock

hardwareTestLock was acquired for
phase10-pi5-local-ls-root-capture-window-proof-20260601, released after the
pre-run boot tree was restored, and ended unlocked/restored.
