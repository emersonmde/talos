# Phase 11 RP1 Endpoint Config Identity Pi 5

Task id: phase11-rp1-endpoint-config-identity-pi5-20260608

Status: accepted

Classification: rp1-endpoint-config-id-all-ones

## Goal

Run the accepted real RP1 endpoint config identity discriminator on Pi 5 and
classify whether the RP1 endpoint identity/config-read layer is observable or
blocked.

## Scope

- Acquired hardwareTestLock for serialized Pi 5 real candidate work.
- Published only the accepted real candidate archive:
  target/talos-rpi5-rp1-endpoint-config-identity-read-core.tar.gz.
- Retained static archive identity, publication identity, fresh serial/TFTP
  cursors, serial output, stable pre-restore TFTP evidence, final pre-restore
  identity, restore evidence, and pi5-capture-transaction-v2 identity join.
- Applied the standard inconclusive-run triage after the first candidate
  capture was rejected by the identity join: candidate identity, fresh
  serial/TFTP evidence, known-good control, and unchanged candidate rerun.

## Non-Goals

No endpoint configuration mutation beyond the accepted EXT_CFG_INDEX selector
write, no BAR programming, bridge setup, PERST or link-control changes,
interrupt enablement or delivery, GIC acknowledgement, ISR installation, RP1
clock/reset writes, GPIO/RIO/pad writes, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, or phase transition.

## Classification

Accepted as rp1-endpoint-config-id-all-ones.

The accepted candidate rerun selected boot tree
7e66c8cef268d7a94843c0d8e230f89c25161053f0b326a8375c0b6f4ca97d42 with
effective kernel_2712.img and a 48,456-byte da591740/kernel_2712.img. The
pi5-capture-transaction-v2 identity join passed with no rejection reasons:
pre-power serial drain was empty, stable pre-restore TFTP retained two served
48,456-byte candidate fetches, final pre-restore identity still matched the
selected tree, and restore returned the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The retained serial output contains 135 occurrences of
TALOS: rp1-endpoint-config-identity-result and 135 occurrences of
classification=rp1-endpoint-config-id-all-ones. The accepted report reads
PCIE_MISC_PCIE_STATUS at 0x1000124068 as raw=0x3e0b0, with pcie-port=true,
dl-active=true, phylinkup=true, link-in-l23=false, and
status-is-deaddead=false. It writes the accepted selector value 0x00100000 to
EXT_CFG_INDEX at 0x1000129000, then reads EXT_CFG_DATA + 0 at 0x1000128000 as
raw-config=0xffffffff, vendor-id=0xffff, and device-id=0xffff.

This accepts only that the bounded source-contract endpoint config identity
attempt reaches the PCIe2 host-link-up precondition and returns an all-ones
config dword for BDF 0002:01:00.0 offset 0. It does not accept endpoint
ownership, expected RP1 vendor/device visibility, endpoint configuration
mutation, BAR programming, bridge setup, interrupt delivery, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, or phase transition.
Same-shaped endpoint config identity hardware reruns are blocked until a
different discriminator or explicit supervisor plan is accepted.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around serialized Pi 5 real
  candidate work.
- fixed: retained static archive identity for the accepted real candidate
  archive, including archive SHA-256, kernel SHA-256, marker string, report
  shape, and accepted PCIE_MISC_PCIE_STATUS/EXT_CFG_INDEX/EXT_CFG_DATA fields.
- fixed: retained the first candidate capture as capture-staging-blocked
  evidence; it had serial marker output but no expected candidate fetches in
  the same-cursor TFTP delta, so no decisive classification was taken.
- fixed: ran the required known-good production-timer control after the
  inconclusive candidate evidence; it passed the identity join with two served
  104,136-byte known-good kernel fetches and retained PASS output.
- fixed: reran the unchanged endpoint config identity candidate after the
  known-good control; the rerun passed the v2 identity join and retained
  repeated rp1-endpoint-config-id-all-ones output.
- deferred: endpoint ownership, expected RP1 vendor/device visibility, bridge
  setup, PERST/link control, interrupt delivery, DMA/cache, storage,
  generated-root, networking, SSH, Milestone 11.3, and phase transition.
- not-an-issue: all-ones config data is an accepted source-contract
  classification and a real frontier result, not grounds to infer broader RP1
  mapping or endpoint ownership.

No findings were removed in this task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-pi5/classification.json.
- Accepted candidate rerun:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-pi5/candidate-rerun-after-kg/.
- First candidate capture:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-pi5/candidate-run/.
- Known-good control:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-pi5/known-good-control-after-inconclusive/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the accepted
  candidate rerun.
- pi5-capture-transaction-v2 identity join: passed on the accepted candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 48,456-byte
  candidate kernel fetches were retained.
- serial hardware boot/output: passed; 135 occurrences of
  TALOS: rp1-endpoint-config-identity-result were retained with classification
  rp1-endpoint-config-id-all-ones.
- known-good control and unchanged candidate rerun after inconclusive evidence:
  run and retained.
- restore proof: passed; final restore returned the lab to the pre-run boot
  tree.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as rp1-endpoint-config-id-all-ones. The queued closeout is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored and supervisorIntervention remains inactive.
