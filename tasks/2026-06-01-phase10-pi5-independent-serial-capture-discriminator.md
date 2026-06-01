# Phase 10 Pi 5 Independent Serial Capture Discriminator

Task: phase10-pi5-independent-serial-capture-discriminator-20260601
Status: accepted-capture-live

## Goal

Determine whether the Pi 5 lab serial capture path can produce fresh retained
bytes through an independent discriminator before any ls-root candidate rerun.

## Scope

This task used a non-ls-root production-timer control archive as a new
serial-capture discriminator:

- control archive: target/talos-rpi5-production-timer-preemption-boot.tar.gz
- archive review:
  tasks/evidence/2026-06-01-pi5-independent-serial-capture-discriminator/local1-production-timer-control/archive-review.txt
- archive sha256:
  tasks/evidence/2026-06-01-pi5-independent-serial-capture-discriminator/local1-production-timer-control/archive-sha256.txt
- kernel sha256:
  tasks/evidence/2026-06-01-pi5-independent-serial-capture-discriminator/local1-production-timer-control/archive-kernel-sha256.txt
- kernel size:
  tasks/evidence/2026-06-01-pi5-independent-serial-capture-discriminator/local1-production-timer-control/archive-kernel-size.txt

No Talos source, proof-code, ls-root script, command-loop behavior, archive
construction, or runtime behavior was changed, and the ls-root candidate was not
rerun.

## Discriminator Choice

The previous blocked-control task proved that an accepted prompt-capable Talos
control could be published and fetched, but serial observe retained no new bytes.
This task therefore used a qualitatively different discriminator: the
production-timer boot tree, which is non-ls-root and expected to be serial-noisy
even before Talos prompt-level responsiveness.

No documented lab serial-capture reset endpoint or independent serial capture
path was available to the worker through the lab-controller API. The available
new discriminator was the non-ls-root control archive plus a fresh power-cycle
from a fresh serial cursor.

## Evidence

Evidence directory:

- tasks/evidence/2026-06-01-pi5-independent-serial-capture-discriminator/local1-production-timer-control/

Key files:

- lab health/status/snapshot inventory:
  - lab-health.json
  - pre-status.json
  - pre-snapshots.json
- prior blocked-control reference:
  tasks/2026-06-01-phase10-pi5-serial-responsive-control-recovery.md
- pre-run cursors:
  - serial-cursor-before.txt
  - tftp-cursor-before.txt
  - pre-serial-peek.json
  - pre-tftp-tail.json
- boot preservation and restore:
  - pre-run-snapshot.json
  - pre-run-snapshot-name.txt
  - restore-snapshot.json
  - post-restore-status.json
- discriminator run:
  - publish.json
  - post-publish-status.json
  - power-cycle.json
  - tftp-delta-after-power.json
  - serial-observe-after-power.json
  - serial-transcript.txt
  - serial-capture-key-lines.txt
  - discriminator-result.txt

## Result

Classification: accepted-capture-live.

The fresh serial cursor advanced from 3840874 to 3841582 and retained 708 bytes
after the production-timer control power cycle. The transcript includes fresh
Raspberry Pi firmware/RP1 boot output, including:

    RPi: BOOTSYS release VERSION:2226a853 DATE: 2025/12/08 TIME: 19:29:54
    BOOTMODE: 0x06 ... serial da591740 ... stc 1096514
    PMIC reset-event 00000000 rtc 6a1d5716
    RP1 FW: load 0

This proves the lab serial capture path can retain new bytes again. It proves
capture liveness only; it does not prove Talos prompt-level responsiveness and
does not unblock the paused ls-root proof by itself.

TFTP did not advance during the short discriminator window:
cursor_start=3992316, cursor_end=3992316, events=0. The serial evidence is
still decisive for capture liveness because the fresh retained bytes appeared
from the new pre-run cursor after the power-cycle.

The boot tree was restored to the pre-run snapshot
pre-phase10-pi5-independent-serial-capture-discriminator-20260601T095629Z.
Post-restore status reported tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Hardware Lock

- owner task: phase10-pi5-independent-serial-capture-discriminator-20260601
- acquired: 2026-06-01T09:55:44Z
- released: 2026-06-01T09:57:29Z
- restore status: PASS; hardwareTestLock released/unlocked/restored in durable
  supervisor state.

## Validation

- static archive/image review: scripts/rpi5-archive-review.sh passed for the
  selected production-timer control archive.
- lab-controller API: health/status/snapshot inventory, pre-run serial cursor,
  and pre-run TFTP cursor retained.
- serial hardware boot/output: fresh firmware/RP1 bytes retained from cursor
  3840874 after the power-cycle; classification accepted-capture-live.
- TFTP: no new events from cursor 3992316 during the short discriminator
  window.
- post-run restore proof: named snapshot restore returned the pre-run tree hash.
- static inspection: git diff --check result retained in the commit workflow.
- documentation: mdBook was run because this task record was added.
- staged static inspection: git diff --cached --check run before commit.

Validation logs:

- tasks/evidence/2026-06-01-pi5-independent-serial-capture-discriminator/validation/git-diff-check.txt
- tasks/evidence/2026-06-01-pi5-independent-serial-capture-discriminator/validation/mdbook-build.txt
- tasks/evidence/2026-06-01-pi5-independent-serial-capture-discriminator/validation/validation-summary.txt

## Next Recommendation

Promote phase10-pi5-prompt-control-after-capture-recovery-20260601 next if the
hardware lock remains unlocked/restored. That task should prove prompt-level
Talos responsiveness with an accepted prompt-capable control before the paused
ls-root proof is resumed.
