# Task: Phase 11 RP1 GPIO Ownership/Restore Pi 5

Task ID: phase11-rp1-gpio-ownership-restore-pi5-20260607

Status: accepted

Evidence level: static archive identity inspection, lab-controller API, serial
hardware boot/output, stable same-cursor TFTP evidence,
pi5-capture-transaction-v2 identity join, restore proof

## Goal

Run the accepted real GPIO ownership/restore diagnostic on Pi 5 after the
paired no-MMIO control proof, retaining decisive identity-joined evidence or a
blocker.

## Scope

- Acquired hardwareTestLock after the no-MMIO control proof was accepted.
- Checked the accepted real candidate archive before publication:
  target/talos-rpi5-rp1-gpio14-ownership-route-preflight-read-core.tar.gz.
- Published only the accepted real GPIO14 ownership/route preflight archive.
- Retained publication identity, fresh serial/TFTP cursors, serial capture,
  stable pre-restore TFTP evidence, final pre-restore identity, restore
  evidence, and v2 identity-join records.
- Performed the required triage after the first powered candidate run was
  rejected by non-empty pre-power serial drain evidence: retained candidate
  identity/TFTP evidence, ran a known-good control, and reran the candidate.

## Non-Goals

No operation outside the accepted source-contract diagnostic boundary,
interrupt delivery, GIC IAR/EOIR acknowledgement, ISR installation, broad GPIO
driver ownership, unplanned pin-control/pad/RIO writes, unplanned clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, or phase transition.

## Classification

Accepted as gpio14-ownership-preflight-blocked-non-gpio-function.

The decisive rerun selected boot tree
91372af6aeecc90b47b57d6d3f1caf46ee5b20f47ec392977fdae2674ac0112f with
effective kernel_2712.img and a 50056-byte da591740/kernel_2712.img. The v2
identity join passed with no rejection reasons: pre-power serial drain was
empty, stable pre-restore TFTP retained two served 50056-byte candidate
fetches, final pre-restore identity still matched the selected tree, and the
capture retained 93 occurrences of
TALOS: rp1-gpio14-ownership-route-preflight-result.

The visible diagnostic line reported contract
phase11-rp1-gpio-ownership-restore-source-contract-v1, target
rp1-gpio14-ownership-route-preflight-read, pin GPIO14, GPIO14 STATUS/CTRL at
0x1f000d0070/0x1f000d0074, IO_BANK0 INTE/INTS at
0x1f000d011c/0x1f000d0124, RIO0 OUT/OE/IN at
0x1f000e0000/0x1f000e0004/0x1f000e0008, pad control at 0x1f000f003c,
INTID 160 GIC status reads, GPIO14 fsel 13 / func-name unknown, RIO GPIO14
out/oe/in true, pad input disabled and output disabled, INTID160 not enabled,
pending, or active, HPPIR INTID 1023, and classification
gpio14-ownership-preflight-blocked-non-gpio-function.

The accepted claim is limited to read-only GPIO14 ownership/route preflight
snapshot visibility and the observed blocker. GPIO ownership, GPIO event
generation, interrupt pending generation beyond the read-only snapshot,
interrupt enablement or delivery, GIC acknowledgement, ISR/handler ownership,
GPIO CTRL/INTE/RIO/pad writes, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe behavior, Milestone 11.3, and
phase transition remain unaccepted.

The capture helper restored its pre-run snapshot after the decisive rerun,
returning the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  diagnostic work.
- fixed: retained static archive identity for the accepted real diagnostic
  archive, including archive SHA-256
  7ccb204d6c14f0b2ad6d9c3796ec4fe000956d98ab25c02a014f75d01184f40e,
  kernel SHA-256
  cb4155be67ee9188dda9f0f17c55afd3e539381e96e35afdec74a2a3a2ebdc19,
  and kernel size 50056 bytes.
- fixed: first candidate run retained 94 diagnostic markers and two matching
  50056-byte candidate TFTP fetches, but was rejected as capture-staging-blocked
  because pre-power serial drain was non-empty.
- fixed: known-good timer control passed the v2 identity join with two
  matching 104136-byte restored-tree TFTP fetches and an empty pre-power drain.
- fixed: candidate rerun passed the repaired v2 identity join and retained the
  accepted diagnostic blocker classification.
- deferred: any future event-generation retry must be replanned around GPIO14
  ownership, function selection, parent-route masking, and restore semantics.
- not-an-issue: the raw 0xdeaddead diagnostic values remain accepted only as
  read-only snapshot visibility at this boundary; they do not imply GPIO
  ownership, event generation, interrupt delivery, or handler ownership.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-pi5/classification.json.
- Decisive candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-pi5/real-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-pi5/real-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-pi5/known-good-control-run/.
- Static archive identity:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-pi5/static/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun as a source/core diagnostic blocker classification.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 50056-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 93 occurrences of
  TALOS: rp1-gpio14-ownership-route-preflight-result were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.

## Result

Accepted as gpio14-ownership-preflight-blocked-non-gpio-function. The queued
closeout task is mechanically unblocked on a future worker wake if
hardwareTestLock remains unlocked/restored and supervisorIntervention remains
inactive.
