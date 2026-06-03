# Phase 10 Pi 5 Local Cat CWD Proof

Task: phase10-pi5-local-cat-cwd-proof-20260602
Status: blocked-proof-control-mismatch

## Goal

Carry the accepted bounded `cat banner.txt` from `/etc` local command feature
to serialized Raspberry Pi 5 serial hardware evidence.

## Scope

- Publish only the accepted archive
  `target/talos-rpi5-local-cat-cwd-candidate-archive-core.tar.gz`.
- Capture candidate identity, fresh serial cursor, settled same-cursor TFTP
  evidence, serial transcript, and restore proof.
- Restore the pre-run boot tree and release `hardwareTestLock`.

## Non-Goals

- No runtime semantic changes.
- No broad relative path traversal, POSIX filesystem/syscall behavior,
  userspace shell execution, networking, SSH, RP1/PCIe, UART interrupt
  ownership, or DMA/cache policy.

## Evidence

### local1-candidate

Evidence directory:
`tasks/evidence/2026-06-02-pi5-local-cat-cwd-proof/local1-candidate/`.

Static/archive and lab-controller evidence:

- Archive sha256:
  `a1159da288089df9cbbf17edc2289d7900108be7864b675bf8291d6352e62c83`.
- Selected kernel: `kernel_2712.img`.
- Candidate tree hash:
  `ae7dcbef171d8e64d67f0783c6253c8860822f91e43689214e7ea3f0a4659d95`.
- Settled same-cursor TFTP evidence showed 13 events, including
  `da591740/kernel_2712.img` served twice at 110992 bytes before restore.
- Restore returned the boot tree to
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

Serial outcome:

- The proof harness sent `cd /etc`, `cat banner.txt`, `cd /`, and
  `cat banner.txt` before Talos reached the prompt. Retained serial output
  shows firmware/network boot progress rather than the cat-cwd command loop.
- No `cat-cwd-observed`, `cat-cwd-negative-observed`,
  `pi5-local-cat-cwd-complete`, or `rpi5-local-cat-cwd-proof: PASS` was
  retained.

Classification: inconclusive proof-run timing failure, not an implementation
failure. The candidate was fetched by TFTP, but the automation advanced on early
firmware bytes and wrote commands before prompt readiness.

### local2-restored-control

Evidence directory:
`tasks/evidence/2026-06-02-pi5-local-cat-cwd-proof/local2-restored-control/`.

The required triage control was attempted after restoring the saved pre-run
snapshot. It proved lab network boot and serial capture health, but it did not
serve as a prompt-responsive local command control:

- Serial retained firmware network boot output, Talos entry, and
  `rpi5-production-timer-preemption: PASS`.
- It did not retain `rpi5-local-ls-cwd-proof: PASS`.
- The restored tree hash was
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, with a
  104136-byte `kernel_2712.img`.

Classification: control mismatch. The saved pre-run tree is production-timer,
not a prompt-responsive ls-cwd/local-command proof image.

### cleanup

Evidence directory:
`tasks/evidence/2026-06-02-pi5-local-cat-cwd-proof/cleanup/`.

The stuck `serial/observe` wrapper was terminated, the saved pre-run snapshot
`pre-cat-cwd-20260603T0109Z` was explicitly restored, and
`hardwareTestLock` was released. Post-cleanup status retained:

- Tree hash:
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- Effective kernel: `kernel_2712.img`.
- Kernel size: 104136 bytes.

## Validation Gates

- Static archive review: `scripts/rpi5-archive-review.sh
  target/talos-rpi5-local-cat-cwd-candidate-archive-core.tar.gz` passed in
  local1 evidence.
- Lab-controller API: candidate publish/status, power-cycle, settled TFTP,
  restore, and post-cleanup status retained.
- Serial hardware boot/output: local1 retained firmware/network boot output but
  no accepted cat-cwd proof; local2 retained production-timer PASS control
  output.
- Restore proof: post-cleanup lab status confirms the saved pre-run tree was
  restored.

## Next Action

Supervisor review is required before another hardware attempt. Valid next
directions are to rerun cat-cwd with a prompt-gated proof harness directly,
identify a prompt-responsive local-command control snapshot, or revise the proof
procedure. Do not change cat-cwd runtime semantics based on this run.
