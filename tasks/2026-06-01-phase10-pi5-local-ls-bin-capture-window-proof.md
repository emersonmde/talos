# Phase 10 Pi 5 Local Ls Bin Capture-Window Proof Task

Task: phase10-pi5-local-ls-bin-capture-window-proof-20260601

Status: blocked-proof-harness-gap

## Goal

Run one unchanged Raspberry Pi 5 'ls /bin' candidate proof with a retained
serial window that begins before the prompt/write boundary and captures visible
init output plus next-prompt readiness.

## Scope

This task did not change Talos runtime semantics, command-loop behavior, parser
behavior, the accepted read-only initramfs fixture, or the candidate archive. It
reused the accepted phase10-local-ls-bin-core candidate:

- target/talos-rpi5-local-ls-bin-local1.tar.gz
- archive sha256:
  209df687cf1312183c8849b66252130d296c0196e97d1476514ffe7cef78e390
- kernel sha256:
  d34ba744c58c2135e937f2ea385ebc39b6d7a81498d67bf08ef827023b9b5f2d

## Evidence

Primary evidence directory:

- tasks/evidence/2026-06-01-pi5-local-ls-bin-proof/

Capture-window attempts:

- local3-capture-window-proof/
- local4-capture-window-proof-corrected-cursor/
- local5-capture-window-proof-final/
- local6-capture-window-proof-rolling-cursor/
- local7-capture-window-proof-read-settle/
- local8-capture-window-proof-read-loop/

The final bounded attempt is local8. It used serial/read chunks to wait for the
fresh prompt, wrote 'ls /bin' immediately, retained the full serial window from
the pre-power cursor, captured TFTP delta, and restored the pre-run boot tree.

## Result

local8/proof-result.txt classified the run as inconclusive:

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

The final serial evidence proves a fresh Pi 5 boot, descriptor-backed fd0/stdout
markers, a ready prompt, final pi5-local-ls-bin-complete classification, and
PASS. It does not retain visible 'ls /bin' input or visible 'init' output:

- tasks/evidence/2026-06-01-pi5-local-ls-bin-proof/local8-capture-window-proof-read-loop/serial-full-window-after-write.clean.txt
- tasks/evidence/2026-06-01-pi5-local-ls-bin-proof/local8-capture-window-proof-read-loop/proof-result.txt

Static source inspection shows why an unchanged rerun is unlikely to satisfy
the current visible-init acceptance gate: src/target/rpi5.rs has replay helpers
and observation markers for ls-root, literal echo, help, pwd, line cancel, and
line kill, but no ls-bin replay/observed marker. The hardware run reaches PASS
from the internal dispatch check, while the retained serial path repeatedly
misses the fast first response bytes before 'truncated=false controls=0'.

## Validation

- image/archive inspection: scripts/rpi5-archive-review.sh passed for the
  unchanged ls /bin candidate archive.
- serial hardware boot/output: local8 retained a full pre-power-to-final serial
  window, but the required visible input and visible init output were absent.
- lab-controller API: TFTP delta includes served kernel_2712.img for the
  candidate tree.
- restore proof: local8 restored the pre-run tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  hardware lock release.
- static inspection: src/target/rpi5.rs lacks an ls-bin-specific visible replay
  or observed marker comparable to ls-root.

## Blocker

The queued task forbids runtime/proof-code semantic changes and requires
visible input plus visible init evidence. The unchanged candidate repeatedly
proves command completion and PASS but cannot satisfy that visible evidence
gate. Supervisor planning is required to decide whether to authorize a bounded
proof-harness visibility fix, adjust the acceptance strategy, or choose the next
feature-led local interactivity task.

## Hardware Lock

hardwareTestLock was acquired for this task, and the lab boot tree was restored
to the pre-run accepted tree before release.
