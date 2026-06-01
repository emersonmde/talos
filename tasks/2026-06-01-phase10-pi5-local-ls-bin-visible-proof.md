# Phase 10 Pi 5 Local Ls Bin Visible Proof Task

Task: phase10-pi5-local-ls-bin-visible-proof-20260601

Status: blocked-proof-input-window

## Goal

Carry the accepted visibility-repaired `ls /bin` proof candidate to serialized
Raspberry Pi 5 evidence, retaining a hardware transcript with the command
response, bounded proof visibility marker, ready-next state, final
`pi5-local-ls-bin-complete` classification, and
`rpi5-local-ls-bin-proof: PASS`.

## Scope

This task did not change Talos runtime command semantics, parser behavior,
the read-only initramfs fixture, or the proof-harness visibility code accepted
by phase10-local-ls-bin-pi5-visible-proof-harness-core-20260601.

The visibility-repaired candidate was built from commit
`0cece48a9e3d03b6f55f444d81c2cbccff06dacb`:

- archive: `target/talos-rpi5-local-ls-bin-visible-proof-local1.tar.gz`
- archive sha256:
  `48391252906d613ccb189e6e9d934646aa54da4404b1d0795d3bccd1046e47a2`
- kernel sha256:
  `514d309d4f3d692c559a5bf7fbad7bcae438858b013ae83f6ad330db3c6c5eed`

Static image review confirmed the candidate contains:

- `rpi5-local-ls-bin-proof`
- `ls-bin-observed input='ls /bin' entries='init'`
- `pi5-local-ls-bin-complete`

## Evidence

Evidence root:

- `tasks/evidence/2026-06-01-pi5-local-ls-bin-visible-proof/`

Hardware attempts and discriminator:

- `local1-visible-proof/`: initial visibility-repaired candidate run. The
  candidate booted and restored, but the worker wrote `ls /bin` after the
  proof had already timed out. Result: inconclusive.
- `local2-fixed-delay-write/`: unchanged candidate rerun with fixed-delay
  serial write. TFTP proved a fresh candidate fetch, but retained serial
  advanced only NUL/newline from the previous cursor. Result: inconclusive.
- `local3-restored-tree-control/`: restored-tree known-good control after
  the serial gap. TFTP and serial both advanced and Talos/control output was
  retained. Result: `control-serial-live`.
- `local4-dtb-scan-trigger-write/`: unchanged candidate rerun after the
  live control, with `ls /bin` written immediately after the Talos
  `dtb memory scan start` boundary was retained. The candidate fetched and
  booted into Talos, but retained output stopped after the dtb scan/NUL and
  did not reach the proof dispatch/PASS lines. Result: inconclusive.

Key summaries:

- `local1-visible-proof/proof-result.txt`
- `local2-fixed-delay-write/proof-result.txt`
- `local3-restored-tree-control/control-result.txt`
- `local4-dtb-scan-trigger-write/proof-result.txt`

## Result

The task is not accepted. The candidate archive identity, image strings,
TFTP staging, and post-run restores are documented, and a restored-tree control
proved the lab serial path can still retain fresh output. However, the
visibility-repaired candidate did not produce a retained Pi 5 transcript that
satisfies the acceptance gate.

The best current discriminator is local4: after a live restored-tree control,
the unchanged candidate fetched and booted, the worker wrote `ls /bin` at
the earliest retained Talos boundary, and the retained output stopped after:

~~~text
TALOS: dtb memory scan start
\0
~~~

It did not retain descriptor-backed fd0/stdout markers, the
`ls-bin-observed` marker, `init`, ready-next, final PASS, or the expected
classification.

## Validation

- lab-controller API: pre/post status and boot file snapshots retained for
  each hardware attempt.
- image/archive inspection: `scripts/rpi5-archive-review.sh` passed for the
  visibility-repaired candidate archive.
- image string inspection: candidate kernel contains the bounded ls-bin proof
  marker and final classification strings.
- serial hardware boot/output: local1, local2, and local4 are inconclusive;
  local3 restored-tree control proves fresh serial capture was possible
  between candidate attempts.
- TFTP evidence: local2, local3, and local4 include fresh served boot-file
  evidence from the lab TFTP log.
- restore proof: every attempt restored the pre-run tree hash
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
- git diff checks: `git diff --check` and `git diff --cached --check`
  passed before commit.

## Blocker

Further progress requires supervisor planning. The worker should not invent a
new proof strategy inside this task, because the accepted hardware gate still
requires a retained Pi 5 transcript with proof-visible command response and
PASS evidence, and additional runtime/proof-harness changes would broaden the
accepted proof-harness core.

Supervisor should decide whether to authorize a narrower Pi 5 input-window
proof strategy, adjust the proof harness to remove the timing race, or choose
the next feature-led local interactivity task.

## Hardware Lock

hardwareTestLock was held for this task while running local1 through local4.
The lab boot tree was restored to the pre-run accepted tree after each run and
before lock release.
