# Phase 11 RP1 Observed Aperture Pi 5

Task id: phase11-rp1-observed-aperture-pi5-20260608

Status: accepted

Classification: observed-aperture-rp1-uart0-fr-visible

## Goal

Run the real observed-aperture read-only discriminator on Pi 5 to decide
whether the accepted bridge/setup mismatch points to a different live RP1
aperture or a retained blocker.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 real candidate work.
- Published only the accepted real observed-aperture archive:
  target/talos-rpi5-rp1-observed-aperture-read-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after the first candidate
  capture: candidate identity, fresh serial/TFTP evidence, known-good controls,
  and unchanged candidate reruns.

## Non-Goals

No endpoint ownership claim, broad RP1 mapping claim, endpoint config retry,
same-shaped 0x1f RP1 read rerun, same-shaped bridge/setup rerun, BAR discovery
or programming, bridge setup writes, PERST/link-control changes, GPIO/pad/
clock/reset writes, interrupt enablement or delivery, GIC acknowledgement,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or phase
transition.

## Classification

Accepted as observed-aperture-rp1-uart0-fr-visible.

The accepted unchanged candidate rerun after known-good-control triage
selected boot tree
def82f95b6ee4440de8014a275cbdef3b1baa4d578d9773e30ff7f15cd2d8a87 with
effective kernel_2712.img and a 47,664-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
47,664-byte candidate fetches, final pre-restore identity still matched the
selected tree, and restore returned the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The retained serial output contains 69 task-owned occurrences of
TALOS: rp1-observed-aperture-result. The report names contract
phase11-rp1-observed-aperture-source-contract-v1 and target
rp1-uart0-fr-observed-aperture-read, reads source RP1 bus address
0xc040030018 through observed CPU physical address 0x1c00030018 at register
offset 0x18, and returns raw=0x187. The result is not 0xdeaddead, not all
ones, not zero, and raw-is-pl011-fr-shaped=true under the local PL011 FR mask.
The terminal classification is observed-aperture-rp1-uart0-fr-visible.

This accepts only visibility for the selected one-read observed aperture and
its report shape. It does not accept endpoint ownership, broad RP1 mapping,
UART ownership, interrupt delivery, GPIO/clock ownership, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  observed-aperture work.
- fixed: retained static archive identity for the accepted real archive,
  including archive SHA-256, kernel SHA-256, kernel size, and result marker.
- fixed: retained the first helper-capture attempt as capture-staging-blocked
  evidence; it selected the candidate tree, but identity join rejected it for
  non-empty pre-power serial drain, saturated direct-read freshness, TFTP byte
  mismatch, and final pre-restore identity mismatch after manual restore.
- fixed: ran the required known-good production-timer controls before
  accepting a candidate rerun. The known-good controls emitted PASS and
  retained two 104,136-byte candidate TFTP fetches, but their identity joins
  remained capture-staging-blocked because the restored production-timer
  baseline continuously emitted serial output and could not satisfy the empty
  pre-power drain predicate.
- fixed: reran the unchanged real observed-aperture candidate after known-good
  control triage; the final rerun passed pi5-capture-transaction-v2 identity
  join with no rejection reasons.
- fixed: accepted the final unchanged candidate rerun as decisive
  observed-aperture visibility with raw=0x187 and raw-is-pl011-fr-shaped=true.
- deferred: endpoint ownership, broad RP1 mapping, UART ownership, interrupt
  delivery, GPIO/clock ownership, DMA/cache, storage, generated-root,
  networking, SSH, Milestone 11.3, and phase transition require later
  supervisor-planned tasks.
- not-an-issue: repeated serial result records are expected because the
  candidate loops its report for capture stability.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/classification.json.
- Accepted manual powered run:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/real-rerun-after-clean-kg/.
- Initial capture-staging-blocked real run:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/real-run/.
- Known-good control triage:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/known-good-control-after-inconclusive/,
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/known-good-control-clean-after-inconclusive/.
- Earlier clean real candidate run retained before completing known-good
  triage:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/real-run-manual/,
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/real-rerun-after-kg/.
- Static archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/static-archive-review.txt.

## Validation

- static archive identity check: passed against the accepted real archive.
- lab-controller serialized Pi 5 hardware run: passed on the accepted final
  unchanged candidate rerun after known-good-control triage.
- pi5-capture-transaction-v2 identity join: passed on the accepted final
  unchanged candidate rerun with no rejection reasons.
- known-good controls after initial inconclusive capture: run and retained;
  PASS marker and matching 104,136-byte TFTP fetches were visible, while
  identity join rejected the controls for non-empty pre-power serial drain on
  the continuously emitting production-timer baseline.
- stable same-cursor TFTP evidence before restore: passed; two 47,664-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 69 task-owned occurrences of
  TALOS: rp1-observed-aperture-result were retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as observed-aperture-rp1-uart0-fr-visible on the final unchanged
candidate rerun after known-good-control triage. The observed 0x1c00030018
aperture is visible for this one selected RP1 UART0 FR read, but ownership and
broader mapping claims remain unaccepted. The queued observed-aperture
closeout is mechanically unblocked on a future worker wake if hardwareTestLock
remains unlocked/restored.
