# Phase 11 RP1 Clock Sentinel Address Discriminator Pi 5

Task id: phase11-rp1-clock-sentinel-address-discriminator-pi5-20260608

Status: accepted

Classification: rp1-sysinfo-and-clock-window-sentinel

## Goal

Run the accepted read-only SYSINFO identity versus retained clock-window
sentinel discriminator on Pi 5.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 work.
- Published only the accepted real candidate archive:
  target/talos-rpi5-rp1-sysinfo-clock-sentinel-read-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after the first powered
  candidate run was rejected by capture evidence: candidate identity, fresh
  serial/TFTP evidence, known-good control, and unchanged candidate rerun.

## Non-Goals

No RP1 clock/reset writes, reset-controller writes, GPIO/RIO/pad writes, event
generation, interrupt enablement or delivery, GIC IAR/EOIR acknowledgement,
ISR installation, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe enumeration, Milestone 11.3, phase transition, or broad RP1 clock/reset
ownership acceptance.

## Classification

Accepted as rp1-sysinfo-and-clock-window-sentinel.

The accepted candidate rerun selected boot tree
22c13cf75878b9f1776d9ae00b760457df45a508b915c3032f4ac792693a74a4 with
effective kernel_2712.img and a 47,776-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
47,776-byte candidate fetches, final pre-restore identity still matched the
selected tree, and restore returned the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The retained serial output contains 62 occurrences of
TALOS: rp1-sysinfo-clock-sentinel-result. Each reported SYSINFO_CHIP_ID,
SYSINFO_PLATFORM, and CLK_ADC_CTRL as 0xdeaddead, with
chip-id-matches-expected=false, chip-id-is-deaddead=true,
platform-is-deaddead=true, adc-ctrl-is-deaddead=true, sysinfo-pair-equal=true,
sysinfo-vs-adc-same=true, retained ADC-window classification
rp1-clock-adc-window-readback-sentinel, and terminal classification
rp1-sysinfo-and-clock-window-sentinel.

This accepts only that the read-only SYSINFO identity/address-decode path has
the same sentinel shape as the retained clock-window comparator on Pi 5. It
does not accept live RP1 SYSINFO identity, broad clock/reset ownership,
clock/reset writes, GPIO ownership, event generation, interrupt delivery, GIC
acknowledgement, ISR/handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe behavior, Milestone 11.3, or a phase transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  candidate work.
- fixed: retained static archive identity for the accepted real candidate
  archive, including archive SHA-256, kernel SHA-256, marker string, report
  shape, and accepted read-only RP1 SYSINFO/CLK_ADC_CTRL address fields.
- fixed: retained the first candidate run as capture-staging-blocked evidence;
  it had 62 result marker occurrences, coherent selected-tree/TFTP evidence,
  and restore proof, but was rejected by non-empty pre-power serial drain.
- fixed: ran the required known-good production-timer control after the
  inconclusive candidate run; it retained two served 104,136-byte known-good
  kernel fetches and reached its PASS marker, but was also rejected by
  non-empty pre-power serial drain.
- fixed: reran the same real SYSINFO-vs-clock-sentinel candidate after the
  known-good control; the rerun passed the v2 identity join and retained the
  repeated rp1-sysinfo-and-clock-window-sentinel output.
- deferred: interpreting the broader SYSINFO/address-decode sentinel boundary
  and selecting any GPIO ownership retry, interrupt-delivery slice, or broader
  clock/reset step require closeout and supervisor planning.
- not-an-issue: retained Raspberry Pi firmware RP1_BOOT chip ID lines still
  report 0x20001927, but Talos read-only SYSINFO loads returned 0xdeaddead and
  are the accepted discriminator result.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5/candidate-rerun-after-kg/.
- First candidate run:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5/candidate-run/.
- Known-good control:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted candidate
  rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 47,776-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 62 occurrences of
  TALOS: rp1-sysinfo-clock-sentinel-result were retained with classification
  rp1-sysinfo-and-clock-window-sentinel.
- known-good control and unchanged candidate rerun after inconclusive evidence:
  run and retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as rp1-sysinfo-and-clock-window-sentinel. The queued closeout is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored and supervisorIntervention remains inactive.
