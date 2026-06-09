# Phase 11 RP1 Clock/Reset Dependency Pi 5

Task id: phase11-rp1-clock-reset-dependency-pi5-20260609

Status: accepted

Classification: observed-clock-reset-dependency-blocked-system-clock-disabled

## Goal

Run the real read-only RP1 clock/reset dependency preflight on Pi 5 and
classify the observed dependency state or blocker.

## Scope

- Acquired the hardware lock for this task only.
- Rebuilt and static-reviewed only the accepted real read-only clock/reset
  dependency candidate archive.
- Published the real candidate and captured candidate identity, fresh serial
  cursor/drain evidence, stable TFTP delta, final selected-tree identity, real
  output, V3/boot-staging checker output, and restore proof.
- Accepted only the selected read-only dependency snapshot classification; no
  write path, interrupt path, DMA/cache, networking, SSH, Milestone 11.3, phase
  transition, or broad RP1 ownership claim is accepted.

## Non-Goals

No clock/reset writes, GPIO function changes, GPIO/RIO/pad/INTE/CTRL writes,
IRQRESET, interrupt unmasking, IAR/EOIR acknowledgement, ISR/handler install,
event generation, interrupt delivery, endpoint config retry, bridge setup
write, DMA/cache, networking, SSH, Milestone 11.3, phase transition, or broad
RP1 ownership claim was attempted or accepted.

## Findings And Disposition

- fixed: static archive review passed for the real read-only preflight archive
  with the accepted source-contract strings and result marker.
- fixed: selected candidate identity before power matched tree
  ef7b62b81d097a52bda724d2173c982fa512e2b6541541514abebd6d8db1422f,
  effective kernel kernel_2712.img, and expected
  da591740/kernel_2712.img size 49,496 bytes.
- fixed: stable pre-restore TFTP evidence retained 13 events with two matching
  da591740/kernel_2712.img fetches of 49,496 bytes.
- fixed: V3 serial freshness passed because the required result marker was
  absent before power and appeared 30 times after power.
- fixed: final pre-restore identity still pointed at the selected tree, and
  restore returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: boot-staging identity checker classified the retained bundle as
  boot-staging-identity-ready with no rejection reasons.
- fixed: marker-visible read-only output classified the dependency state as
  observed-clock-reset-dependency-blocked-system-clock-disabled:
  chip-id-matches-expected=true, pll-sys-locked=true, clk-sys-enabled=false,
  clk-slow-sys-enabled=false, clk-uart-enabled=true, and no selected clock
  returned the 0xdead_dead sentinel.
- deferred: any clock/reset write, GPIO function change, event generation,
  interrupt delivery, DMA/cache, networking, SSH, and phase-transition work
  remains outside this task.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-pi5/classification.json.
- Static archive review:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-pi5/static-archive-review.txt.
- Capture bundle:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-pi5/real-run/.
- V3 checker:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-pi5/v3-check.json.
- Boot-staging identity checker:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-pi5/boot-staging-identity-check.json.
- Marker excerpt:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-pi5/marker-first-line.txt.
- Restore proof:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-pi5/real-run/post-restore-status.json.

## Validation

- image/archive inspection: static review passed for the real read-only
  preflight archive; archive SHA-256
  98f43bab73199eba86308a8e86a113dc9dd87598e277ddd95481d9a8da2b84cc.
- lab-controller API: health, snapshot, archive publish, power-cycle, serial,
  TFTP, final identity, and restore records were captured.
- serial hardware boot/output: V3 passed; required marker occurrences were 0
  before power and 30 after power.
- TFTP evidence: passed; stable same-cursor pre-restore delta retained two
  matching 49,496-byte candidate kernel fetches.
- final pre-restore identity: passed; selected tree remained staged before
  restore.
- restore proof: passed; post-restore tree hash matched the pre-run baseline.
- capture/staging checker output: V3 and boot-staging identity checkers passed
  with no rejection reasons.
- jq empty on evidence-map, classification, checker, and retained JSON
  artifacts: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed before commit.

No docs/src files were touched, so mdbook is not required.

## Result

Accepted as observed-clock-reset-dependency-blocked-system-clock-disabled. This
accepts only the read-only RP1 clock/reset dependency snapshot classification:
the observed sysinfo and PLL reads are visible, selected clock-manager reads
are not sentinel values, and the selected system clock enable fields are not
set in this snapshot.

## Next Action

The next queued worker task is
phase11-rp1-clock-reset-dependency-closeout-20260609, mechanically unblocked if
this task is committed and hardwareTestLock remains unlocked/restored.
