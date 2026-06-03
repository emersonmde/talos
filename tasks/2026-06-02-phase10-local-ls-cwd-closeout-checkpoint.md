# Phase 10 Local Ls Cwd Closeout Checkpoint Task

Task: phase10-local-ls-cwd-closeout-checkpoint-20260602

Status: accepted

## Goal

Close out the bounded local bare `ls` current-directory feature after
accepted QEMU/substitute evidence, RPi5 candidate archive evidence, and
serialized Pi 5 feature proof.

## Scope

This was documentation-only closeout work. It reconciled accepted evidence,
recorded the command-context cwd listing frontier, named deferred POSIX,
filesystem, userspace shell, and hardware-driver surfaces, and handed planning
back to the supervisor for the next feature-led local interactivity task.

No code changed. No QEMU scenario was rerun. No boot archive was published. No
Pi 5 hardware action, power-cycle, or hardwareTestLock acquisition occurred.

Changed files:

- docs/src/SUMMARY.md
- docs/src/project/phase10-local-ls-cwd-closeout-checkpoint.md
- docs/src/roadmap.md
- tasks/2026-06-02-phase10-local-ls-cwd-closeout-checkpoint.md

## Accepted Frontier

Accepted: bare `ls` resolves against command-context cwd for `/`, `/etc`,
and `/bin` over the descriptor-backed serial command loop and returns to a
ready prompt. The accepted sequence is `pwd`, bare `ls` at `/`,
`cd /etc`, bare `ls` with `banner.txt`, `cd /bin`, bare `ls` with
`init`, `cd /`, bare `ls` with root entries, and `bogus` regression.

This remains bounded kernel-backed shell UX. It does not accept broad path
traversal, POSIX cwd/syscalls, descriptor-backed filesystem syscalls,
process-local cwd inheritance, userspace shell execution, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache policy.

## Evidence

- QEMU/substitute bare `ls` cwd evidence:
  tasks/evidence/2026-06-02-qemu-local-ls-cwd-core/qemu-local-ls-cwd-smoke.log.
- RPi5 candidate archive evidence:
  tasks/evidence/2026-06-02-rpi5-local-ls-cwd-candidate-archive-core/.
- Pi 5 proof summary:
  tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/proof-result.txt.
- Pi 5 serial transcript:
  tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/serial-transcript.txt.
- Pi 5 settled TFTP proof:
  tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/tftp-delta-settled-before-restore.json.
- Pi 5 restore proof:
  tasks/evidence/2026-06-02-pi5-local-ls-cwd-proof/local2-candidate/post-restore-status.json.
- Closeout doc:
  docs/src/project/phase10-local-ls-cwd-closeout-checkpoint.md.

Accepted prior commits:

- QEMU/substitute core: 4a3db877499328b10e75bff9f1eb3bc36f7579ae.
- RPi5 candidate archive: 742f0eaba91bd4986d8fa456722de89f94aa7015.
- Pi 5 proof: b17832a7232d74c3d4f90dde677c4beb86271945.

## Validation

- static inspection: reviewed accepted task records and retained evidence paths.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- pre-commit static inspection: `git diff --cached --check` passed.
- hardware lock: hardwareTestLock remained unlocked/restored and unused.

Acceptance commit: recorded in durable supervisor state after commit creation.

## Next Action

No explicit mechanically unblocked feature task remains after this closeout.
Set planningNeeded=true and return selection of the next smallest
feature-led local interactivity task to the supervisor. Do not create a new
task, infer a phase transition, or promote blocked proof chains.
