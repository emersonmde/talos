# Phase 11 RP1 Observed GPIO Status Pi 5

Task id: phase11-rp1-observed-gpio-status-pi5-20260608

Status: completed-blocker

Classification: capture-staging-blocked

## Goal

Run the real read-only observed-aperture GPIO14 STATUS/CTRL preflight on Pi 5
and classify whether the 0x1c observed aperture reaches those GPIO registers.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 work.
- Published only the accepted real archive:
  target/talos-rpi5-rp1-observed-gpio-status-read-core.tar.gz.
- Captured candidate identity, fresh serial/TFTP evidence, serial markers,
  TFTP deltas, final selected-tree identity, restore evidence, and
  pi5-capture-transaction-v2 identity joins.
- Performed the required known-good production-timer control after the first
  real candidate run was inconclusive.

## Non-Goals

No endpoint ownership claim, broad RP1 mapping claim, endpoint config retry,
BAR discovery or programming, bridge setup writes, PERST/link-control changes,
GPIO/pad/clock/reset writes, interrupt enablement or delivery, GIC
acknowledgement, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, phase transition, source-contract expansion, or serial
freshness relaxation.

## Classification

Completed as capture-staging-blocked.

The real candidate selected boot tree
52b5f11000b24f6f6d00ab1b9aaa4d62a4d4114486a0302ad593b713a08c2559 with
effective kernel_2712.img and two 49,656-byte da591740/kernel_2712.img TFTP
fetches. Serial output retained 42 task-owned
TALOS: rp1-observed-gpio-status-result records with marker-visible report
values gpio14-status-raw=0xabe3300, gpio14-ctrl-raw=0x84, ctrl-funcsel=4, and
terminal classification=observed-aperture-gpio14-status-ctrl-visible.

That marker-visible result is not accepted as a decisive RP1 hardware
classification because the repaired freshness gate failed before the power
cycle: the pre-power drain exhausted all 96 attempts, read 1,095,168 bytes, and
never reached empty-read-before-power. pi5-capture-transaction-v2 rejected the
run for serial-drain-not-empty-before-power and
saturated-direct-read-without-empty-pre-power-drain. The run still retained
stable TFTP evidence, final selected-tree identity, and restore to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The required known-good production-timer control selected tree
407d10f6ed4457e89f9023f769c00920a4ebbe0f42ca65b0165b8db014140697 with two
104,136-byte da591740/kernel_2712.img TFTP fetches, but failed the same
repaired freshness discriminator: 96 pre-power serial-drain attempts,
1,095,168 bytes, no empty-read-before-power, and the same v2 rejection
reasons. Per task policy, the unchanged real candidate was not rerun after
the known-good control failed the same repaired freshness gate.

This task therefore accepts only a blocker: the real GPIO14 STATUS/CTRL
candidate and known-good control both remain non-decisive under the repaired
serial freshness contract. It does not accept GPIO14 STATUS/CTRL visibility,
GPIO ownership, event generation, interrupt pending generation, interrupt
delivery, endpoint ownership, broad RP1 mapping, pad/RIO/clock/reset
ownership, DMA/cache, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 work.
- fixed: retained static archive identity for the accepted real archive.
- fixed: retained candidate identity, serial drain, serial output, TFTP delta,
  final selected-tree identity, and restore proof for the real candidate.
- fixed: retained known-good production-timer control evidence after the
  inconclusive real candidate.
- fixed: classified the real candidate and known-good control as the same
  repaired freshness blocker rather than accepting stale saturated serial as
  decisive hardware evidence.
- not-an-issue: the marker-visible real serial report is retained as evidence
  but is not accepted because the pre-power drain did not prove freshness.
- deferred: new capture-repair planning or another supervisor-approved
  discriminator is required before same-shaped real GPIO14 STATUS/CTRL reruns.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-pi5/classification.json.
- Real candidate run:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-pi5/real-run/.
- Known-good control after inconclusive candidate:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-pi5/known-good-control-after-inconclusive/.
- Static archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-pi5/static-archive-review.txt.

## Validation

- static archive identity check: passed against the real archive.
- lab-controller API candidate identity check before power-cycle: passed for
  the real candidate and known-good control.
- serial hardware boot/output: both runs retained their required markers.
- stable same-cursor TFTP evidence before restore: passed for both runs.
- pi5-capture-transaction-v2 identity join: rejected both runs for
  serial-drain-not-empty-before-power and
  saturated-direct-read-without-empty-pre-power-drain.
- restore proof: passed; both runs restored to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Completed as a committed blocker. The closeout task is mechanically unblocked
after this blocker is committed and hardwareTestLock remains unlocked/restored.
