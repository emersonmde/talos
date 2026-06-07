# Phase 11 RP1 GPIO Bank Source-Status Pi 5

Task id: phase11-rp1-gpio-bank-source-status-pi5-20260607

Status: accepted

## Goal

Run the real read-only RP1 GPIO bank source-status diagnostic on Pi 5 after the
paired no-MMIO/no-RP1/no-GIC control proof.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 diagnostic work.
- Checked the accepted real candidate archive before publication:
  target/talos-rpi5-rp1-gpio-bank-source-status-read-core.tar.gz.
- Published only the accepted real GPIO bank source-status diagnostic archive.
- Retained publication identity, fresh serial/TFTP cursors, serial capture,
  stable pre-restore TFTP evidence, final pre-restore identity, restore
  evidence, and v2 identity-join records.
- Performed the required triage after the first powered candidate run was
  rejected by non-empty pre-power serial drain evidence: retained candidate
  identity/TFTP evidence, ran a known-good control, and reran the candidate.

## Non-Goals

No GPIO event generation, GPIO interrupt enablement, GPIO CTRL/IRQRESET writes,
MSI-X, PCIe MIP, or GIC writes, interrupt delivery, IAR/EOIR acknowledgement,
ISR installation, GPIO ownership, pin-control writes, clock/reset programming,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, phase transition, or acceptance of behavior beyond the
read-only IO_BANK0 INTE/INTS source-status snapshot boundary.

## Classification

Accepted as gpio-bank-source-status-visible.

The decisive rerun selected boot tree
84ee89db45d5298e49f44c74e6a18b9c07ce2c146879f677aceace6ad252ea0f with effective
kernel_2712.img and a 46904-byte da591740/kernel_2712.img. The v2 identity join
passed with no rejection reasons: pre-power serial drain was empty, stable
pre-restore TFTP retained two served 46904-byte candidate fetches, final
pre-restore identity still matched the selected tree, and the capture retained
269 occurrences of TALOS: rp1-gpio-bank-source-status-result.

The visible diagnostic line reported contract
phase11-rp1-gpio-bank-source-status-contract-v1, target
rp1-io-bank0-source-status-read, source hwirq 0, bank0 GPIO0..GPIO27,
IO_BANK0 INTE at 0x1f000d011c, IO_BANK0 INTS at 0x1f000d0124, width 32,
raw INTE 0xdeaddead, raw INTS 0xdeaddead, GPIO14 mask 0x4000,
gpio14-enabled=true, gpio14-source-status=true, source-status-mask=0xdeaddead,
source-status-nonzero=true, and classification=gpio-bank-source-status-visible.

The accepted claim is limited to read-only GPIO bank source-status snapshot
visibility and report decoding. GPIO event generation, interrupt pending
generation beyond the read-only snapshot, interrupt enablement or delivery,
IAR/EOIR acknowledgement, ISR/handler ownership, GPIO ownership, pin-control
state, clock/reset programming, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe behavior, Milestone 11.3, and phase transition remain
unaccepted.

The capture helper restored its pre-run snapshot after the decisive rerun,
returning the lab to tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  diagnostic work.
- fixed: retained static archive identity for the accepted real diagnostic
  archive, including archive SHA-256, kernel SHA-256, marker string, and
  accepted report shape.
- fixed: retained the first candidate run as capture-staging-blocked evidence;
  it had candidate-tied fetches and 270 diagnostic markers but was rejected by
  non-empty pre-power serial drain evidence.
- fixed: ran the required known-good control after the inconclusive first
  candidate run; it retained the production timer PASS marker and preserved
  the serial-drain rejection evidence.
- fixed: reran the selected real diagnostic after the known-good control; the
  rerun passed the v2 identity join and retained repeated GPIO bank
  source-status output.
- deferred: closeout reconciliation remains queued before supervisor planning
  of any next Milestone 11.2 feature slice.
- not-an-issue: the raw 0xdeaddead values are accepted only as read-only
  snapshot visibility evidence at this diagnostic boundary; they do not imply
  GPIO ownership, event generation, interrupt delivery, or handler ownership.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-pi5/classification.json.
- Decisive candidate rerun:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-pi5/real-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-pi5/real-run/.
- Known-good control:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-pi5/known-good-control-run/.
- Static archive identity:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-pi5/static/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 46904-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 269 occurrences of
  TALOS: rp1-gpio-bank-source-status-result were retained.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as gpio-bank-source-status-visible. The queued closeout task is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored and supervisorIntervention remains inactive.
