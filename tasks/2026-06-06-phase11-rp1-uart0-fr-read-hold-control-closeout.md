# Phase 11 RP1 UART0 FR Read Hold-Control Closeout

Task id: phase11-rp1-uart0-fr-read-hold-control-closeout-20260606

Status: accepted

## Goal

Reconcile the hold-control RP1 UART0 FR-read source/static and Pi 5
discriminator evidence into the exact accepted hardware boundary and next
Phase 11 frontier.

## Scope

- Reviewed the accepted hold-control core evidence and committed Pi 5
  discriminator evidence.
- Recorded findings with disposition.
- Stated the final classification and exact accepted and unaccepted claims.
- Kept the Phase 11 RP1/PCIe map contract proof status unchanged because the
  discriminator already recorded the current capture-staging-blocked boundary.
- Identified the smallest next bounded Phase 11 task as a capture/staging
  discriminator repair planned by the supervisor, not another same-shaped
  FR-read hardware rerun.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime/source changes, GPIO, interrupts, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.2, or phase transition.

This closeout does not convert post-read loop serial text into RP1
mapped/read-value acceptance because that text was not tied to selected-
candidate TFTP fetch evidence.

## Final Classification

capture-staging-blocked.

The source/static core is accepted as a local/archive candidate. It branches
from Pi 5 rust_entry before BootInfo parsing, target initialization, boot
reports, memory planning, allocator setup, scheduler work, or command-loop
work. It emits the unique UART10 pre-read control marker, performs exactly one
contracted 32-bit volatile load from 0x1f00030018, and reports the contract id,
target, address, width, raw value, mapped/read-value classification, and
post-read terminal hold marker if the read returns. The accepted archive is
target/talos-rpi5-rp1-uart0-fr-read-hold-control-core.tar.gz with SHA-256
e9ab45b6dd15e4e80395302a116fb8aa751d699c5b679e5b9cee22077059a9b2; its kernel
image is 46,320 bytes.

The Pi 5 discriminator published that accepted archive under hardware lock.
Publication reported lab tree
ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0,
effective kernel kernel_2712.img, and the expected 46,320-byte
da591740/kernel_2712.img. The main direct-read serial window retained 222,783
bytes with 5,582 TALOS: fr-hold-control-post-read-loop occurrences, but the
stable same-cursor pre-restore TFTP delta recorded 13 restored-tree events with
zero selected 46,320-byte candidate fetches. Because the serial was not
candidate-tied, it cannot accept mapped/read-value, bus-fault/trap, pre-read
control visibility, or candidate-fetch-without-control-marker behavior.

Required triage did not remove the blocker. The known-good control produced
stable same-cursor TFTP with zero events. The candidate rerun again staged the
46,320-byte kernel and reported effective kernel kernel_2712.img, but stable
same-cursor TFTP returned zero events and the serial observe file remained
empty. The lab was restored to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
hardware-lock release.

Accepted claims are limited to the local/static hold-control FR-read candidate,
the hardware-locked publication/capture attempts, restore hygiene, and the
capture-staging-blocked classification. RP1 UART0 FR mapped/read-value
behavior, bus-fault/trap behavior, pre-read-control-visible-without-read-result,
candidate-fetch-without-control-marker, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, and phase transition remain unaccepted.

## Findings And Disposition

- fixed: reconciled the source/static core with the committed Pi 5
  discriminator evidence.
- fixed: preserved the exact capture-staging-blocked hardware classification
  instead of upgrading serial post-read-loop text that lacked selected-candidate
  TFTP fetch evidence.
- fixed: recorded that hardware lock acquisition, publication identity,
  stable pre-restore TFTP, serial capture, triage, restore, validation, and
  commit evidence are retained by the discriminator task.
- deferred: a decisive RP1 FR-read hardware classification still requires a
  capture/staging repair that ties selected-candidate fetch, serial cursor, and
  marker/read/trap output in the same run.
- deferred: the smallest next bounded task requires supervisor planning because
  same-shaped hardware reruns are explicitly rejected by this closeout.
- not-an-issue: the Phase 11 contract document already records the current
  hold-control discriminator boundary, so no docs/src proof-status edit was
  required by this closeout.
- not-an-issue: no hardwareTestLock acquisition was required for this static
  closeout.

## Evidence

- Static closeout inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-closeout/evidence-map.json.
- Core task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core.md.
- Core evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/evidence-map.json.
- Pi 5 discriminator task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator.md.
- Pi 5 discriminator evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/classification.json and
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-pi5-discriminator/evidence-map.json.
- Contract status:
  docs/src/project/phase11-rp1-pcie-map-contract.md.

## Validation

- static evidence inspection: completed.
- git diff --check: passed.
- mdbook build: not run; docs/src files were not touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as capture-staging-blocked.

Supervisor planning is required for the next non-repetitive Phase 11 task. The
next task should repair the capture/staging tie before another RP1 UART0 FR
hardware run is considered, because another same-shaped publish/power/TFTP/
serial rerun would not create durable progress.
