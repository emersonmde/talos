# Phase 10 Pi 5 Local Ls Root Proof Task

Task: phase10-pi5-local-ls-root-proof-20260601

Status: paused-capture-gap-confirmed

## Goal

Carry the accepted bounded `ls /` root-listing command to serialized Raspberry
Pi 5 serial hardware evidence.

## Scope

This task adds only the Pi 5 proof harness and staging scripts needed to publish
the accepted kernel-backed `ls /` feature. The command-loop semantics remain
the accepted local core: exact `ls /` dispatch, descriptor-backed fd0/stdout,
visible read-only initramfs root entries `bin`, `dir`, `empty`, and `etc`,
and next-prompt readiness.

Changed files in the in-progress implementation:

- build.rs
- src/target/rpi5.rs
- scripts/rpi5-local-ls-root-image.sh
- scripts/rpi5-local-ls-root-boot-tree.sh
- tasks/2026-06-01-phase10-pi5-local-ls-root-proof.md
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/

## Current Evidence

Local validation passed before hardware publication:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local1-candidate/local-gates-summary.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local1-candidate/qemu-local-ls-root-smoke.log

Archive identity and static review:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local1-candidate/archive-review.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local1-candidate/archive-sha256.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local1-candidate/archive-kernel-sha256.txt

local1 candidate summary:

```text
archive_sha256=16f5a053e05459239645b96eace01ee7f46139fa558b264af8a336a02d2a112c
kernel_sha256=904eeb9348ff1c0d1ade43c8e441b68f3bf9cef01055b2af2c3ad7d23a82eb24
pre_tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
candidate_tree_hash=697809f9570d28254de04a56fa7b45173e8dfbd06c12fb81b2dfbb2944adae25
post_restore_tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
classification=inconclusive-hardware-run
```

The local1 candidate was fetched over TFTP with the expected 105912-byte
`da591740/kernel_2712.img`, but retained serial output stopped at Raspberry Pi
firmware output plus two later NUL/newline bytes. It did not show the Talos
proof start marker, `talos>`, `ls /`, root entries, final
`pi5-local-ls-root-complete`, or `rpi5-local-ls-root-proof: PASS`.

A known-good accepted literal-echo control archive was then published and
power-cycled. That control also retained only firmware output plus later
NUL/newline bytes after a scripted `echo local serial works` write. TFTP tail
evidence shows the control boot request sequence, but the control did not reach
its accepted Talos prompt/output in retained serial.

Known-good control evidence:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local1-known-good-literal-echo-control/archive-review.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local1-known-good-literal-echo-control/serial-after-power.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local1-known-good-literal-echo-control/serial-command.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local1-known-good-literal-echo-control/tftp-tail-after-no-delta.json
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local1-known-good-literal-echo-control/post-control-rollback-status.json

## Deferred Surfaces

Deferred: broad shell parsing, quoting, globbing, userspace shell execution,
process spawning, writable filesystem state, filesystem syscalls, recursive
listing, terminal/session behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy.

## Resume Note

Do not change ls-root runtime semantics after this inconclusive hardware run
until a supervisor decision or documented lab recovery action produces a
responsive known-good control. The lab boot tree was rolled back to the pre-run
hash `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, and
the hardware lock should be reacquired only for a deliberate control recovery or
unchanged candidate rerun.

## 2026-06-01 Local2/Local3 Resume Attempts

After phase10-pi5-prompt-control-after-capture-recovery-20260601 accepted fresh
prompt-responsive known-good control evidence, this task reran the unchanged
ls-root candidate archive `target/talos-rpi5-local-ls-root-local1.tar.gz`
without changing Talos runtime or proof semantics.

Local validation was rerun before hardware publication:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local2-unchanged-candidate-rerun/local-gates-summary.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local2-unchanged-candidate-rerun/qemu-local-ls-root-smoke.log

local2 recorded the same candidate archive/kernel identity, preserved the
pre-run boot tree, and restored it after the run. The first TFTP query was too
early and showed no delta; a follow-up query after restore captured bootloader
TFTP activity, so its byte attribution is quarantined by the lab-controller
warning about querying TFTP evidence after restore.

local3 repeated the unchanged candidate with an explicit boot/TFTP wait:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local3-unchanged-candidate-rerun/proof-result.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local3-unchanged-candidate-rerun/tftp-delta-before-restore.json
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local3-unchanged-candidate-rerun/serial-transcript-clean.txt

local3 proved fresh TFTP fetches of `da591740/kernel_2712.img` with the
candidate archive still active, but retained serial advanced only NUL/newline
bytes from cursor 3849726 to 3849730. It did not show Talos boot output, the
`talos>` prompt, `ls /`, root entries, `pi5-local-ls-root-complete`, or
`rpi5-local-ls-root-proof: PASS`.

A post-candidate known-good literal-echo control then used the previously
accepted archive `target/talos-rpi5-local-literal-echo-local3.tar.gz`:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local3-known-good-literal-echo-control/control-result.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local3-known-good-literal-echo-control/tftp-delta-before-restore.json
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local3-known-good-literal-echo-control/serial-transcript-clean.txt

That control also fetched `da591740/kernel_2712.img` and restored the pre-run
boot tree, but retained serial again advanced only NUL/newline bytes and did
not produce a prompt, command output, classification, or PASS line. This
invalidates accepting ls-root hardware behavior in this worker wake. The next
step requires supervisor planning for a fresh lab/control discriminator before
any ls-root proof-code or runtime-code change.

## 2026-06-01 Local4 Unchanged Candidate Rerun

After phase10-pi5-serial-write-ingress-control-proof-20260601 accepted fresh
prompt-live serial-write ingress evidence, this task performed exactly one
unchanged rerun of the existing ls-root candidate archive
`target/talos-rpi5-local-ls-root-local1.tar.gz`.

Local validation before publication:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/local-gates-summary.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/qemu-local-ls-root-smoke.log

Candidate identity and static review:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/archive-review.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/archive-sha256.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/archive-kernel-sha256.txt

Hardware evidence:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/proof-result.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/serial-prompt-transcript.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/serial-response-transcript.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/tftp-delta-before-restore.json
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-unchanged-candidate-rerun/post-restore-status.json

local4 proved the unchanged candidate was fetched over TFTP, reached
`rpi5-local-ls-root-proof: ready command=0` and a visible `talos>` prompt,
accepted a lab serial write of `ls /`, and restored the pre-run boot tree hash
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
The retained post-write serial included the final
`classification=pi5-local-ls-root-complete` and
`rpi5-local-ls-root-proof: PASS` lines, but the retained transcript did not
include visible `bin`, `dir`, `empty`, and `etc` entry lines or a complete
`ready-for-next prompt=true` line. Because the task acceptance gate requires
visible root entries in retained Pi 5 evidence, local4 is recorded as
`blocked-or-inconclusive`, not accepted.

No Talos runtime semantics, proof harness code, boot scripts, or lab-controller
code changed in local4. The next step requires supervisor planning around the
retained-response capture gap or an explicit acceptance-policy decision; do not
rerun or modify ls-root proof code from worker initiative alone.

## 2026-06-01 Local4 Response Capture Audit

The bounded response-capture audit task
phase10-pi5-local-ls-root-response-capture-audit-20260601 inspected existing
local4 evidence and read-only serial-log API windows without acquiring
hardwareTestLock or mutating the lab.

Audit task record:

- tasks/2026-06-01-phase10-pi5-local-ls-root-response-capture-audit.md

Audit evidence:

- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-response-capture-audit/audit-summary.txt
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-response-capture-audit/serial-observe-from-retrospective-start.json
- tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-response-capture-audit/serial-observe-from-write-cursor.json

The audit confirmed the local4 capture gap. The broader retained serial window
from cursor 3899942 recovered 358 bytes, and the post-write window from cursor
3900142 recovered 158 bytes. Both windows matched the existing retained local4
evidence: final classification and PASS are present, but visible `bin`, `dir`,
`empty`, and `etc` lines are still absent, and only the suffix
`xt prompt=true` was retained instead of a complete `ready-for-next
prompt=true` line.

The Pi 5 ls-root proof remains not accepted because the visible-entry
acceptance gate is unsatisfied. The next mechanically unblocked task is the
queued unchanged-candidate capture-window proof, which may rerun the existing
candidate exactly once with corrected retained-response capture if
hardwareTestLock remains unlocked/restored.
