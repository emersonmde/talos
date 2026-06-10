# Phase 12 RP1 Ethernet GEM MID Decode Discriminator V2 Closeout

Task: phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-closeout-20260610

Status: accepted

Classification: rp1-ethernet-gem-mid-decode-discriminator-v2-frontier-closed

Evidence level: static inspection of the accepted repaired Pi 5 proof task
record, classification JSON, evidence map, capture summaries, Phase 12 project
docs, roadmap, and git history. No additional Pi 5 hardware run was performed.

## Goal

Close out the repaired v2 GEM MID decode-discriminator hardware proof without
expanding acceptance to Ethernet driver readiness, packet I/O, live DMA,
descriptor rings, interrupts, networking, sockets, SSH, Phase 12.2, or a phase
transition.

## Scope

- Reconciled the accepted
  phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof-20260610
  task record, classification JSON, evidence map, capture summary, project
  doc, roadmap, and commit 112c39e4.
- Preserved the repaired pi5-capture-chain-v4 evidence boundary: selected-tree
  identity, expected TFTP fetch bytes, run-unique serial marker freshness,
  final pre-restore identity, restore proof, and paired no-MMIO/no-Ethernet
  control marker retention.
- Closed same-shaped GEM MID decode-discriminator hardware retries.
- Requested supervisor planning for the next explicit Phase 12.1 slice because
  no queued task is mechanically objective from this sentinel result alone.

## Non-Goals

No runtime source changes, no additional Pi 5 hardware run, no boot archive
publication, no hardwareTestLock acquisition, no Ethernet driver
implementation, no packet I/O, no DMA or descriptor rings, no interrupt
delivery, no clock/reset writes, no PHY reset/MDIO work, no networking, no
sockets, no SSH, no Phase 12.2 work, and no phase transition.

## Reconciled Inputs

- tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof.md
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/classification.json
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/evidence-map.json
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/capture-summary.json
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/candidate-run/identity-join-v4-check.json
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/control-run/identity-join-v4-check.json
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- git history through 112c39e4 Accept GEM MID decode discriminator v2 Pi 5 proof

## Closeout Decision

The repaired v2 proof closes the capture-chain blocker for this discriminator.
Candidate and control runs both passed pi5-capture-chain-v4 identity,
expected-TFTP, run-unique serial marker, final identity, restore, and evidence
map gates. The paired control retained the no-MMIO/no-Ethernet report marker
without constructing RP1 or Ethernet MMIO targets.

Accepted candidate evidence:

- Staged tree:
  99ba865fff0ce6829f0525fb0a20580205e8e12f5deab91139acf45d2d9f15b1
- kernel_2712.img bytes: 49,176
- kernel_2712.img SHA-256:
  255c89d8680b5fd6afff028e5a9aad402aecb3836021d2bc71391c574e7eecc8
- Capture nonce: gemdisc-v2-candidate-20260610T051645Z-b4d146e9
- TFTP evidence: two expected fetches of da591740/kernel_2712.img with
  matching 49,176-byte length.
- Serial evidence: 62 occurrences of the required candidate marker after
  power and zero pre-power nonce occurrences.
- Observed positive control: SYSINFO_CHIP_ID at 0x1c00000000 returned
  0x20001927.
- Observed GEM MID target: translated MACB_MID at 0x1f001000fc returned
  0xdeaddead.
- Classification:
  observed-rp1-positive-control-gem-mid-0x1f-window-sentinel.

Accepted control evidence:

- Staged tree:
  4a52e74fb0e9007d4689053c7296cdc6722e354fff78827f958f8c85870d6494
- kernel_2712.img bytes: 48,432
- kernel_2712.img SHA-256:
  10b962002346cf08972d215d071a3c760a2eea0fac8aa485d5560d3682e20af3
- Capture nonce: gemdisc-v2-control-20260610T051645Z-b4d146e9
- TFTP evidence: two expected fetches with matching 48,432-byte length.
- Serial evidence: 64 occurrences of the required control marker after power
  and zero pre-power nonce occurrences.
- Classification:
  no-mmio-no-ethernet-rp1-ethernet-gem-mid-decode-discriminator-control.

The lab restored to the baseline tree before this closeout:

    a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

Same-shaped GEM MID decode-discriminator hardware retries are closed. Repeating
the same candidate/control proof would only reproduce the accepted boundary:
same-run RP1 SYSINFO is visible at the observed positive-control address while
translated 0x1f GEM MID still returns the retained sentinel. A useful follow-up
requires supervisor planning around a different bounded discriminator or a
bridge/address-window dependency slice with explicit acceptance criteria.

No queued follow-up task is mechanically objective from this closeout alone.
The worker should set planningNeeded=true and stop quietly after accepting this
checkpoint.

## Findings

- fixed: reconciled the accepted v2 proof and recorded that the repaired
  capture chain now supports decisive classification for this discriminator.
- fixed: preserved the candidate positive-control evidence:
  SYSINFO_CHIP_ID at 0x1c00000000 returned 0x20001927.
- fixed: preserved the GEM MID hardware frontier: translated MACB_MID at
  0x1f001000fc returned the retained 0xdeaddead sentinel.
- fixed: documented the paired no-MMIO/no-Ethernet control as visible under
  the same repaired capture-chain gates without constructing prohibited RP1 or
  Ethernet MMIO targets.
- fixed: closed same-shaped GEM MID decode-discriminator retries unless future
  supervisor scope supplies a materially different discriminator, bridge/window
  evidence, or acceptance criteria.
- deferred: PCIe/RP1 bridge or address-window enablement, Ethernet clock/reset
  ownership, PHY reset/MDIO ownership, live GEM visibility, packet I/O, DMA,
  descriptor rings, interrupts, networking, sockets, SSH, Phase 12.2, and
  phase transition remain future work.
- not-an-issue: no additional hardware run was required because this closeout
  is a static checkpoint over committed Pi 5 evidence.

No findings were removed.

## Rejected Claims And Retained Risks

This closeout does not accept live GEM visibility, broad Ethernet MMIO
readiness, Ethernet driver readiness, RP1 MMIO/DMA programming, descriptor
rings, DMA ownership, transfer completion, interrupt completion, clock/reset
ownership, PHY reset/MDIO ownership, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

Retained risks:

- PCIe/RP1 bridge or address-window enablement remains unaccepted.
- Ethernet clock/reset and PHY/MDIO ownership remain unaccepted.
- Live GEM visibility and broad Ethernet MMIO readiness remain unaccepted.
- DMA, descriptor rings, interrupts, packet I/O, networking, sockets, SSH, and
  Phase 12.2 remain out of scope.

## Evidence

- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-closeout/evidence-map.json.
- Accepted v2 proof task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof.md.
- Accepted v2 proof classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/classification.json.
- Accepted v2 proof evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/evidence-map.json.
- Accepted v2 proof capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-v2-pi5-proof/capture-summary.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- static inspection: reviewed v2 proof task record, classification JSON,
  evidence map, capture summary, candidate/control identity joins, Phase 12
  project doc, roadmap, and git history.
- JSON checks: jq empty on task-owned evidence-map/classification JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles v2 hardware proof/blocker without expanding acceptance
  to Ethernet driver readiness, packet I/O, live DMA, descriptor rings,
  interrupts, networking, sockets, SSH, Phase 12.2, or phase transition:
  satisfied.
- Checkpoint states whether same-shaped GEM MID decode-discriminator hardware
  retries are closed, blocked, or require a different discriminator:
  satisfied; closed unless future supervisor scope supplies a materially
  different discriminator, bridge/window evidence, or acceptance criteria.
- NextAction selects one bounded follow-up only if mechanically objective;
  otherwise planningNeeded=true with a concrete blocker reason: satisfied;
  supervisor planning is required because no queued follow-up is mechanically
  objective from the retained 0x1f GEM MID sentinel.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Supervisor planning required. Do not repeat the same GEM MID
decode-discriminator hardware proof, start Ethernet driver implementation,
program RP1 MMIO/DMA, create descriptor rings, claim interrupts/clock/reset/PHY
ownership, perform packet I/O, add networking/sockets/SSH, start Phase 12.2, or
infer a phase transition from this checkpoint. A future task needs explicit
scope and acceptance criteria for a different bounded discriminator or a
bridge/address-window dependency slice.
