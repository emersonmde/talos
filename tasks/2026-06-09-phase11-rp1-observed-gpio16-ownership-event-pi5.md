# Phase 11 RP1 Observed GPIO16 Ownership/Event Pi 5

Task id: phase11-rp1-observed-gpio16-ownership-event-pi5-20260609

Status: accepted

Classification: observed-gpio16-ownership-preflight-blocked-non-gpio-function

## Goal

Run the real read-only observed-aperture GPIO16 ownership/event preflight on
Pi 5 under decisive staging and capture evidence.

## Scope

- Acquired the hardware lock for this task only.
- Published only the task-owned real read-only preflight archive.
- Ran the accepted GPIO16 observed-aperture STATUS/CTRL, IO_BANK0 INTE/INTS,
  RIO OUT/OE/IN, pad-control, and parent GIC route status reads.
- Captured selected candidate identity, fresh serial cursor/drain evidence,
  stable TFTP delta, final pre-restore identity, V3 capture checker output,
  boot-staging identity checker output, and restore proof.

## Non-Goals

No GPIO/RIO/pad/INTE/CTRL writes, IRQRESET, interrupt unmasking, IAR/EOIR
acknowledgement, ISR/handler install, event generation, interrupt delivery,
GPIO14 ownership change, endpoint config retry, bridge setup write,
clock/reset write, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: static archive review passed for the real read-only preflight archive
  with the accepted source-contract strings and result marker.
- fixed: selected candidate identity before power matched tree 908eadd18fab1ba826d2dba92125649383a4857ed39ea18af125feb721a637c3
  with expected fetch da591740/kernel_2712.img at 50640 bytes.
- fixed: V3 serial freshness passed because the required result marker was
  absent before power (0 occurrences) and present after power
  (89 occurrences).
- fixed: stable pre-restore TFTP evidence retained 13 events with
  two matching da591740/kernel_2712.img fetches of 50640 bytes.
- fixed: final pre-restore identity still pointed at the selected tree, and
  restore returned the lab to baseline tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: rpi5-proof-identity-join-v3-check.sh classified the bundle as
  capture-transaction-v3-ready with no rejection reasons.
- fixed: rpi5-boot-staging-identity-check.sh classified the bundle as
  boot-staging-identity-ready with no rejection reasons.
- fixed: marker-visible serial output included 89 occurrences
  of the required result marker.
- fixed: the marker-visible preflight classified GPIO16 as a non-GPIO-function
  blocker: funcsel=31, func-name=unknown,
  pad-input-enable=false, pad-output-disable=true, and
  HPPIR reporting INTID 1023.
- deferred: GPIO ownership, event generation, pending generation, interrupt
  delivery, and any write-based transition remain outside this task.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-pi5/classification.json.
- Static archive review:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-pi5/static-archive-review.txt.
- Capture bundle:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-pi5/real-run/.
- V3 checker:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-pi5/v3-check.json.
- Boot-staging identity checker:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-pi5/boot-staging-identity-check.json.
- Marker excerpt:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-pi5/marker-first-line.txt.
- Restore proof:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-pi5/real-run/post-restore-status.json.

## Validation

- image/archive inspection: static review passed for the real read-only
  preflight archive; archive SHA-256 ae8f97043e61976bcc64dbebd48ae90cbd352d2ad1aeb245a6be3723247cb680.
- lab-controller API: health, snapshot, archive publish, power-cycle, serial,
  TFTP, final identity, and restore records were captured.
- serial hardware boot/output: V3 passed; required marker occurrences were
  0 before power and 89 after power.
- TFTP evidence: passed; stable same-cursor pre-restore delta retained two
  matching 50640-byte candidate kernel fetches.
- final pre-restore identity: passed; selected tree remained staged before
  restore.
- restore proof: passed; post-restore tree hash matched the pre-run baseline.
- capture/staging checker output: V3 and boot-staging identity checkers passed
  with no rejection reasons.
- jq empty on evidence-map, classification, checker, and retained JSON
  artifacts: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

No docs/src files were touched, so mdbook is not required.

## Result

Accepted as observed-gpio16-ownership-preflight-blocked-non-gpio-function. This accepts only read-only GPIO16 preflight
visibility/classification: GPIO16 is currently not a GPIO-owned event target
under this observed aperture. The run does not accept GPIO ownership, event
generation, interrupt delivery, handler ownership, broad RP1 mapping,
DMA/cache, networking, SSH, Milestone 11.3, or a phase transition.

## Next Action

The next queued worker task is
phase11-rp1-observed-gpio16-ownership-event-closeout-20260609, mechanically
unblocked if this task is committed and hardwareTestLock remains
unlocked/restored.
