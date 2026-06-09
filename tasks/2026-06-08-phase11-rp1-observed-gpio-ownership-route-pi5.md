# Phase 11 RP1 Observed GPIO14 Ownership/Route Pi 5

Task id: phase11-rp1-observed-gpio-ownership-route-pi5-20260608

Status: accepted

Classification: observed-gpio14-ownership-preflight-blocked-non-gpio-function

## Goal

Run the real read-only observed-aperture GPIO14 ownership/route preflight on
Pi 5 under decisive staging and capture evidence.

## Scope

- Acquired the hardware lock for this task only.
- Published only the task-owned real read-only preflight archive.
- Ran the accepted read-only observed-aperture GPIO14 ownership/route
  preflight loads and retained marker-visible serial output.
- Captured selected candidate identity, fresh serial cursor/drain evidence,
  stable TFTP delta, final pre-restore identity, V3 capture checker output,
  boot-staging identity checker output, and restore proof.

## Non-Goals

No GPIO/RIO/pad/INTE/CTRL writes, IRQRESET, interrupt unmasking, IAR/EOIR
acknowledgement, ISR/handler install, event generation, interrupt delivery,
endpoint config retry, bridge setup write, clock/reset write, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: static archive review passed for the real read-only preflight archive
  with the accepted source-contract strings and result marker.
- fixed: selected candidate identity before power matched tree
  e6ded87c576967c770223930463864fc081443467d6e00fbe108f29fa9e33fd2 with
  effective kernel kernel_2712.img and expected fetch
  da591740/kernel_2712.img at 50,496 bytes.
- fixed: pre-power serial drain saturated without an empty read, but V3
  freshness passed because the required result marker was absent before power
  and appeared 142 times after power.
- fixed: stable pre-restore TFTP evidence retained 13 events with two matching
  da591740/kernel_2712.img fetches of 50,496 bytes.
- fixed: final pre-restore identity still pointed at the selected tree with the
  expected fetch present at 50,496 bytes.
- fixed: restore returned the lab to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: rpi5-proof-identity-join-v3-check.sh classified the bundle as
  capture-transaction-v3-ready with no rejection reasons.
- fixed: rpi5-boot-staging-identity-check.sh classified the bundle as
  boot-staging-identity-ready with no rejection reasons.
- fixed: the marker-visible preflight classified GPIO14 as a non-GPIO-function
  blocker: funcsel=4, func-name=uart0, GPIO14 input sampled high, INTID160 not
  enabled, pending, or active, and HPPIR reporting 1023.
- not-an-issue: capture-invariant-summary.json retains the older V2
  identity-join-mismatch suggestion because V2 requires an empty pre-power
  serial drain; the accepted V3 checker is the decisive freshness gate for this
  saturated-cursor run.
- deferred: GPIO ownership, event generation, pending generation, interrupt
  delivery, and any write-based transition remain outside this task.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5/classification.json.
- Static archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5/static-archive-review.txt.
- Capture bundle:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5/real-run/.
- V3 checker:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5/v3-check.json.
- Boot-staging identity checker:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5/boot-staging-identity-check.json.
- Marker excerpt:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5/marker-first-line.txt.
- Restore proof:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5/real-run/post-restore-status.json.

## Validation

- image/archive inspection: static review passed for the real read-only
  preflight archive.
- lab-controller API: health, snapshot, archive publish, power-cycle, serial,
  TFTP, final identity, and restore records were captured.
- serial hardware boot/output: passed; the required result marker was absent
  before power and present after power.
- TFTP evidence: passed; stable same-cursor pre-restore delta retained two
  matching 50,496-byte candidate kernel fetches.
- final pre-restore identity: passed; selected tree remained staged before
  restore.
- restore proof: passed; post-restore tree hash matched the pre-run baseline.
- capture/staging checker output: V3 and boot-staging identity checkers passed
  with no rejection reasons.
- jq empty on evidence-map, classification, checker, and retained JSON
  artifacts: passed.
- git diff --check: passed.

No docs/src files were touched, so mdbook was not required.

## Result

Accepted as observed-gpio14-ownership-preflight-blocked-non-gpio-function. This
accepts only read-only preflight visibility/classification: GPIO14 is currently
function 4, UART0, not a GPIO-owned event-generation target. The run does not
accept GPIO ownership, event generation, interrupt delivery, handler ownership,
broad RP1 mapping, DMA/cache, networking, SSH, Milestone 11.3, or a phase
transition.

## Next Action

The next queued worker task is
phase11-rp1-observed-gpio-ownership-route-closeout-20260608, mechanically
unblocked if this task is committed and hardwareTestLock remains
unlocked/restored.
