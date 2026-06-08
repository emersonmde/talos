# Phase 11 RP1 Observed GPIO Status Control Pi 5

Task id: phase11-rp1-observed-gpio-status-control-pi5-20260608

Status: completed-blocker

Classification: capture-staging-blocked

## Goal

Run the paired no-MMIO/no-RP1/no-GIC observed-aperture GPIO status control on
Pi 5 to prove output/capture shape before the real observed-aperture read.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 control work.
- Published only the accepted no-MMIO/no-RP1/no-GIC observed GPIO status
  control archive:
  target/talos-rpi5-rp1-observed-gpio-status-no-mmio-control-core.tar.gz.
- Captured candidate identity, fresh serial/TFTP cursors, serial output,
  stable pre-restore TFTP evidence, final selected-tree identity, and restore
  evidence.
- After the first run was inconclusive, followed the standard triage sequence:
  candidate identity, fresh serial/TFTP evidence, known-good production-timer
  control, and no code changes before classification.

## Non-Goals

No real observed-aperture GPIO MMIO read, endpoint ownership claim, broad RP1
mapping claim, endpoint config retry, BAR discovery or programming, bridge
setup writes, PERST/link-control changes, GPIO/pad/clock/reset writes,
interrupt enablement or delivery, GIC acknowledgement, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, phase transition, runtime
source changes, or source-contract expansion.

## Classification

Completed as capture-staging-blocked.

The first candidate run selected boot tree
133f2a9b4a4c5c21b206d8f9eb8eba4ffe41f787a69b33bced0ea164a8cf83ab with
effective kernel_2712.img and a 48,952-byte da591740/kernel_2712.img. Stable
pre-restore TFTP retained two served 48,952-byte candidate fetches, and serial
output retained 41 occurrences of TALOS: rp1-observed-gpio-status-control plus
41 terminal no-mmio-observed-gpio-status-control-visible classifications.

The pi5-capture-transaction-v2 identity join rejected the first candidate
evidence with serial-drain-not-empty-before-power and
saturated-direct-read-without-empty-pre-power-drain. The pre-power serial drain
made 16 attempts, accumulated 182,528 bytes, saturated at cursor 4,194,304, and
did not reach an empty /serial/read response. Because the saturated direct-read
serial window was not proven fresh, the serial/TFTP evidence cannot accept the
control proof even though the expected candidate image was fetched and markers
were visible.

The required known-good control triage selected boot tree
407d10f6ed4457e89f9023f769c00920a4ebbe0f42ca65b0165b8db014140697 with
effective kernel_2712.img and a 104,136-byte da591740/kernel_2712.img. Stable
pre-restore TFTP retained two served 104,136-byte known-good fetches, and
serial output retained one rpi5-production-timer-preemption: PASS marker. The
known-good identity join failed for the same serial freshness reasons:
serial-drain-not-empty-before-power and
saturated-direct-read-without-empty-pre-power-drain.

The lab restored to baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 after both
runs, and hardwareTestLock was released/restored.

This result accepts only a capture-chain blocker. It does not accept the GPIO
status control proof, the real observed-aperture GPIO14 read, GPIO ownership,
interrupt delivery, broad RP1 mapping, DMA/cache, networking, SSH,
Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around the serialized Pi 5
  work.
- fixed: retained static archive identity for the accepted no-MMIO observed
  GPIO status control archive.
- fixed: retained the first candidate capture with candidate identity, fresh
  serial/TFTP cursors, serial output, stable pre-restore TFTP evidence, final
  selected-tree identity, and restore proof.
- fixed: retained known-good production-timer control triage after the
  inconclusive candidate evidence.
- deferred: no unchanged candidate rerun was performed after known-good
  because the known-good proof itself failed the same serial freshness gate.
- deferred: a future worker/supervisor task must repair or re-plan the serial
  drain/capture freshness path before this control proof can be accepted.
- not-an-issue: the candidate and known-good TFTP fetches were observed with
  matching byte counts before restore; the blocker is the serial freshness
  contract, not archive publication or restore.
- not-an-issue: no real RP1 aperture, endpoint, interrupt, GPIO, clock/reset,
  or DMA behavior is inferred from this no-MMIO control blocker.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5/classification.json.
- First candidate run:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5/control-run/.
- Known-good control after inconclusive candidate:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-control-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted no-MMIO control
  archive.
- lab-controller serialized Pi 5 hardware run: completed for the candidate and
  known-good control; both restored to the baseline tree.
- pi5-capture-transaction-v2 identity join: failed for both retained runs with
  serial-drain-not-empty-before-power and
  saturated-direct-read-without-empty-pre-power-drain.
- stable same-cursor TFTP evidence before restore: passed for both retained
  runs; the candidate had two 48,952-byte fetches and known-good had two
  104,136-byte fetches.
- serial hardware boot/output: candidate retained 41 task-owned control
  markers; known-good retained one production-timer PASS marker.
- restore proof: passed; both restores returned the lab to the pre-run baseline
  tree.
- git diff --check: passed.
- mdbook build: not required unless docs/src files are touched.
- git diff --cached --check before commit: passed.

## Result

Completed as a committed capture-staging-blocked task. No later real GPIO
status Pi 5 proof is mechanically unblocked by this result.
