# Phase 12 RP1 Ethernet GEM MID Decode Discriminator Closeout

Task: phase12-rp1-ethernet-gem-mid-decode-discriminator-closeout-20260610

Status: accepted

Classification: rp1-ethernet-gem-mid-decode-discriminator-closeout-accepted

Evidence level: static inspection of the accepted local/static discriminator
core, task-owned evidence, project docs, and git history. No Pi 5 hardware run
was performed.

## Goal

Close the local/static decode discriminator core and decide whether the
serialized Pi 5 proof is mechanically unblocked.

## Scope

- Consumed accepted
  phase12-rp1-ethernet-gem-mid-decode-discriminator-core-20260610
  implementation, task record, classification JSON, evidence map, project
  docs, and commit.
- Confirmed the discriminator differs from the accepted GEM MID-only proof by
  requiring a same-run observed RP1 positive-control load of
  `SYSINFO_CHIP_ID` at `0x1c00000000` plus the translated `MACB_MID`
  target at `0x1f001000fc`.
- Confirmed the paired control uses the same report contract while
  constructing neither observed RP1 nor Ethernet MMIO targets.
- Selected the queued serialized Pi 5 proof because the candidate/control
  archive shape, hardware lock requirement, evidence gates, and allowed
  classifications are explicit.
- Preserved non-goals against Ethernet driver behavior, packet I/O, DMA,
  descriptor rings, interrupts, networking, sockets, SSH, Phase 12.2 work,
  and phase transition claims.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
Ethernet driver implementation, packet I/O, DMA, descriptor rings, interrupt
delivery, clock/reset writes, PHY reset, networking, sockets, SSH, Phase 12.2
work, or phase transition.

## Closeout Decision

Selected:
`phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof-20260610`.

The selected hardware proof must use this archive shape:

- Candidate archive: same-run read-only observed RP1 positive-control load of
  `SYSINFO_CHIP_ID` at `0x1c00000000`, expected `0x20001927`, plus one
  read-only `MACB_MID` load at `0x1f001000fc`.
- Control archive: same reporting path with no observed RP1 MMIO target and
  no Ethernet MMIO target, carrying the no-MMIO/no-Ethernet discriminator
  control classification.
- Required serialization: acquire hardwareTestLock before boot archive
  publication or Pi 5 hardware interaction, then restore the prior boot
  snapshot and release the lock after evidence capture.
- Required evidence gates: candidate/control identity through lab API
  `GET /`, fresh serial cursor, TFTP delta from `GET /tftp/logs`, serial
  transcript, final pre-restore identity, restore proof, task-owned
  classification JSON, and task-owned evidence map.
- Allowed candidate classifications:
  `observed-rp1-positive-control-gem-mid-0x1f-window-sentinel`,
  `observed-rp1-positive-control-and-gem-mid-visible`,
  `observed-rp1-positive-control-sentinel`, `staging/build-blocker`,
  `inconclusive-capture`, or another precise blocker supported by the
  captured evidence.

This selection does not authorize Ethernet implementation or any broader
hardware behavior. It only authorizes a later serialized proof of the accepted
decode-discriminator report shape.

## Findings

- fixed: closed the accepted local/static discriminator core as a changed
  candidate/control report shape relative to the accepted GEM MID-only proof.
- fixed: selected the queued serialized Pi 5 proof because its candidate
  archive, paired control, hardware lock, evidence gates, and allowed
  classifications are explicit.
- fixed: preserved the no-MMIO/no-Ethernet control boundary and the same-run
  observed positive-control requirement.
- deferred: Pi 5 proof execution, live GEM identity, broad Ethernet MMIO
  readiness, bridge/window enablement, Ethernet clock/reset ownership, PHY
  reset/MDIO ownership, packet I/O, networking, sockets, SSH, Phase 12.2, and
  phase transition work.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  local/static checkpoint only.

No findings were removed.

## Rejected Claims And Retained Risks

This closeout does not accept live GEM visibility, broad Ethernet MMIO
readiness, Ethernet driver readiness, RP1 MMIO/DMA programming, descriptor
rings, transfer completion, interrupt completion, clock/reset ownership, PHY
reset ownership, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

Retained risks:

- the translated `0x1f` RP1-window sentinel source remains unproven;
- PCIe/RP1 bridge or window enablement remains unaccepted;
- Ethernet clock/reset dependency remains unaccepted;
- live GEM identity, broad Ethernet MMIO readiness, PHY reset/MDIO ownership,
  packet I/O, networking, sockets, SSH, and Phase 12.2 remain unaccepted.

## Evidence

- Accepted discriminator core task record:
  `tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-core.md`.
- Accepted discriminator core classification:
  `tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-core/classification.json`.
- Accepted discriminator core evidence map:
  `tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-decode-discriminator-core/evidence-map.json`.
- Implementation: `src/rp1_ethernet.rs`.
- Phase 12 project doc: `docs/src/project/phase12-networking-ssh.md`.
- Current git history:
  `b9628146 Accept GEM MID decode discriminator core`.

## Validation

- static inspection: reviewed accepted discriminator core task record,
  classification JSON, evidence map, implementation, Phase 12 project doc, and
  recent git history.
- diff check: `git diff --check` passed.
- documentation build: `/home/node/.cargo/bin/mdbook build` passed because
  docs/src was touched.
- staged diff check: `git diff --cached --check` passed before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Closeout selects or blocks
  `phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof-20260610`:
  satisfied by selected.
- If hardware proof is selected, nextAction names candidate/control archive
  shape, required hardware lock, evidence gates, and allowed classifications:
  satisfied.
- No driver, packet, DMA, interrupt, networking, sockets, SSH, Phase 12.2, or
  phase transition claim is accepted: satisfied.

## Next Action

Mechanically promote
`phase12-rp1-ethernet-gem-mid-decode-discriminator-pi5-proof-20260610` on
the next worker wake as a serialized Pi 5 decode-discriminator proof only. It
must acquire hardwareTestLock before publishing boot archives or interacting
with Pi 5 hardware, capture candidate/control identity, fresh serial cursor,
TFTP delta, serial transcript, final pre-restore identity, restore proof,
classification JSON, and evidence map, then restore/release the hardware lock.
It must classify the candidate only as
`observed-rp1-positive-control-gem-mid-0x1f-window-sentinel`,
`observed-rp1-positive-control-and-gem-mid-visible`,
`observed-rp1-positive-control-sentinel`, `staging/build-blocker`,
`inconclusive-capture`, or another precise blocker supported by evidence.
It must not implement Ethernet behavior, program RP1 MMIO/DMA, create
descriptor rings, claim interrupts/clock/reset/PHY ownership, perform packet
I/O, add networking/sockets/SSH, start Phase 12.2, or claim a phase
transition.
