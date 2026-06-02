# Phase 10 Pi 5 Local Cat Banner Unchanged Rerun After Control

Task: phase10-pi5-local-cat-banner-unchanged-rerun-after-control-20260602
Status: accepted by supervisor feature-led review

## Goal

Rerun the unchanged accepted `cat /etc/banner.txt` Pi 5 candidate after the
settled TFTP accepted-control discriminator proved the lab prompt path was
responsive again.

## Scope

This task reused the unchanged cat-banner candidate from
`phase10-pi5-local-cat-banner-proof-20260602`:

- candidate archive: `target/talos-rpi5-local-cat-banner-local1.tar.gz`
- source commit: `9301bed6b955c61f5c0bae5ce8b145498538d595`
- archive sha256:
  `35937283006c1079df2d95836343c4cd81e54655989e238fea70aa746778feb0`
- kernel sha256:
  `5300184ebc40ac3b5bb44c9c96828f0d4b1c71b2a8f4431593fff8e5394abce3`

No Talos runtime behavior, command semantics, proof-harness visibility,
parser behavior, filesystem/syscall behavior, userspace execution, process
lifecycle, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy was changed.

## Evidence

Attempt `local3-unchanged-rerun-after-settled-control` retained:

- result summary:
  `tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/proof-result.txt`
- serial transcript:
  `tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/serial-transcript.txt`
- post-run serial tail:
  `tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/serial-peek-post-run-65536.txt`
- TFTP delta:
  `tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/tftp-delta-before-restore.json`
- publish and restore status:
  `tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/post-publish-status.json`
  and
  `tasks/evidence/2026-06-02-pi5-local-cat-banner-proof/local3-unchanged-rerun-after-settled-control/post-restore-status.json`

The candidate archive was staged with `effective_kernel=kernel_2712.img` and
candidate tree hash
`5535fe57219ff7d2873926ba7b443603dd0ae85d00006a1fe31828087c9345d9`.
Settled same-cursor TFTP evidence from cursor `4043654` showed fresh
`da591740/kernel_2712.img` fetches of 107520 bytes, matching the unchanged
candidate kernel size.

Serial hardware evidence reached the feature path. The delayed write of
`cat /etc/banner.txt` retained visible `Talos initramfs fixture` output,
the `cat-banner-observed` marker, a fresh `talos>` ready-for-next prompt,
final `classification=pi5-local-cat-banner-complete`, and the exact
`rpi5-local-cat-banner-proof: PASS` line.

## Blocker

The queued acceptance criteria also required explicit descriptor-backed fd0/stdout
markers in retained Pi 5 serial evidence. The unchanged candidate did not emit
that marker for the `rpi5_local_cat_banner` scenario. Static source inspection
shows the shared Pi 5 command-loop startup marker is currently compiled for
earlier local command scenarios but not for `rpi5_local_cat_banner`.

Because the task scope explicitly required an unchanged rerun and forbade proof
harness visibility changes, the worker cannot mark the task accepted without
relaxing acceptance criteria or changing the candidate/proof harness. Supervisor
planning is required to decide whether the accepted frontier should use the
feature PASS as sufficient evidence or schedule a bounded marker/harness update.

Supervisor feature-led review on 2026-06-02T08:14:00Z accepted the user-visible
feature proof despite the absent descriptor marker for this unchanged scenario.
The accepted frontier uses the retained prompt, visible banner output,
ready-for-next prompt, final classification, exact PASS line, fresh TFTP, and
restore evidence as sufficient Pi 5 proof. Descriptor-backed command-loop
behavior remains covered by the accepted local cat-banner core and prior local
command-loop proof lineage; future marker work is optional and must be
feature-justified.

## Restore

The pre-run tree hash was
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
Post-restore status returned to the same tree hash with
`effective_kernel=kernel_2712.img`. The hardware lock was released and marked
restored in durable supervisor state.

## Validation

- image/archive inspection: `scripts/rpi5-archive-review.sh` passed for the
  unchanged cat-banner candidate archive.
- lab-controller API: health, pre/post status, snapshot, publish, boot files,
  fixed-port power-cycle, serial write/observe, TFTP logs, and restore
  artifacts are retained.
- serial hardware boot/output: retained transcript shows the feature response,
  ready prompt, final classification, and PASS, but no explicit descriptor
  marker for this scenario.
- TFTP evidence: retained delta shows fresh candidate boot-file fetches before
  restore.
- restore proof: pre/post boot tree hashes matched.
- static inspection: `git diff --check` passed.
- documentation: mdBook was not required because mdBook docs were not touched.
